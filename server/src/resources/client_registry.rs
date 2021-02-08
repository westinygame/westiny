use std::fmt;
use std::net::SocketAddr;
use thiserror::Error;

/// An ID that uniquely identifies a network client.
/// Can be used in game logic to match relevant entities to network clients.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct ClientID(u32);

pub struct ClientHandle {
    pub id: ClientID,
    pub addr: SocketAddr,
    /// Right now it is used as a user_name, but no further authentication done.
    pub player_name: String,
}

pub struct ClientRegistry {
    max_slots: usize,
    next_id: u32,
    clients: Vec<ClientHandle>,
}

#[derive(Error, Debug)]
pub enum AddError {
    #[error("Player with already connected with same address xor name, not authorizing. Possibly malicious attempt?")]
    Unauthorized,

    #[error("Server is full")]
    ServerIsFull,
}

#[derive(Error, Debug)]
pub enum RemoveError {
    #[error("No such registered address")]
    NoSuchClient,
}

impl ClientRegistry {
    pub fn new(max_slots: usize) -> Self {
        ClientRegistry {
            max_slots,
            next_id: 0,
            clients: vec![],
        }
    }

    pub fn add(&mut self, addr: &SocketAddr, player_name: &str) -> Result<ClientID, AddError> {
        if self.clients.len() >= self.max_slots {
            return Err(AddError::ServerIsFull);
        }

        match self.find_by_addr_or_name(&addr, player_name) {
            Some(h) if h.player_name == player_name && &h.addr == addr => Ok(h.id),
            Some(_) => Err(AddError::Unauthorized),
            None => Ok(self.add_new_client(*addr, player_name)),
        }
    }

    pub fn find_client(&self, client_id: ClientID) -> Option<&ClientHandle> {
        self.clients.iter().find(|&handle| handle.id == client_id)
    }

    pub fn find_by_addr(&self, addr: &SocketAddr) -> Option<&ClientHandle> {
        self.clients.iter().find(|&handle| &handle.addr == addr)
    }

    pub fn remove(&mut self, addr: &SocketAddr) -> Result<ClientID, RemoveError> {
        if let Some(index) = self.clients.iter().position(|handle| &handle.addr == addr) {
            let removed_id = self.clients[index].id;
            self.clients.remove(index);
            Ok(removed_id)
        } else {
            Err(RemoveError::NoSuchClient)
        }
    }

    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    fn add_new_client(&mut self, addr: SocketAddr, player_name: &str) -> ClientID {
        let id = ClientID(self.next_id);
        self.next_id += 1;
        self.clients.push(ClientHandle {
            id: id,
            addr,
            player_name: player_name.into(),
        });
        id
    }

    fn find_by_addr_or_name(&self, addr: &SocketAddr, name: &str) -> Option<&ClientHandle> {
        self.clients
            .iter()
            .find(|&handle| &handle.addr == addr || handle.player_name == name)
    }
}

impl fmt::Display for ClientRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClientRegistry (count: {})", self.client_count())?;
        for handle in &self.clients {
            write!(
                f,
                "\n  - ID={}, address={}, player_name={}",
                handle.id.0, handle.addr, handle.player_name
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_addr(ip: &str, port: u16) -> SocketAddr {
        SocketAddr::new(
            ip.parse()
                .expect("invalid IP address provided for make_addr"),
            port,
        )
    }

    #[test]
    fn test_registered_client_address_can_be_retrieved() {
        let mut reg = ClientRegistry::new(4);

        let address = make_addr("8.8.8.8", 1234);
        let result = reg.add(&address, "NariFeco");
        assert!(result.is_ok());
        let handle = reg.find_client(result.unwrap()).expect("client not found");
        assert_eq!(handle.addr, address);
        assert_eq!(handle.player_name, "NariFeco");

        let handle_by_addr = reg.find_by_addr(&address).expect("client by address is not found");
        assert_eq!(handle.player_name, handle_by_addr.player_name);
        assert_eq!(handle.id, handle_by_addr.id);
        assert_eq!(handle.addr, handle_by_addr.addr);
    }

    #[test]
    fn test_multiple_clients_can_be_registered() {
        let mut reg = ClientRegistry::new(4);

        let address_one = make_addr("8.8.8.8", 1234);
        let address_two = make_addr("1.1.1.1", 1234);
        let one_id = reg
            .add(&address_one, "NariFeco")
            .expect("Could not add NariFeco :(");
        let two_id = reg
            .add(&address_two, "BananJoe")
            .expect("Could not add BananJoe :(");

        let handle_one = reg.find_client(one_id).expect("player one not found");
        let handle_two = reg.find_client(two_id).expect("player two not found");

        assert_eq!(handle_one.addr, address_one);
        assert_eq!(handle_one.player_name, "NariFeco");

        assert_eq!(handle_two.addr, address_two);
        assert_eq!(handle_two.player_name, "BananJoe");

        assert_eq!(reg.client_count(), 2);
    }

    #[test]
    fn test_querying_non_registered_client_should_return_none() {
        let reg = ClientRegistry::new(4);
        let maybe_handle = reg.find_client(ClientID(42));
        assert!(maybe_handle.is_none());
    }

    #[test]
    fn test_querying_non_registered_client_by_address_should_return_none() {
        let reg = ClientRegistry::new(4);
        let maybe_handle = reg.find_by_addr(&make_addr("1.1.1.1", 42));
        assert!(maybe_handle.is_none());
    }

    #[test]
    fn test_registering_same_client_again_should_return_its_id() {
        let mut reg = ClientRegistry::new(4);
        let address = make_addr("8.8.8.8", 1234);
        let id = reg
            .add(&address, "NariFeco")
            .expect("could not add NariFeco");

        let result = reg.add(&address, "NariFeco");
        let id_again = result.expect("NariFeco added.");
        assert_eq!(id, id_again);
    }

    #[test]
    fn test_registering_same_client_with_different_name_should_return_error() {
        let mut reg = ClientRegistry::new(4);
        let address = make_addr("8.8.8.8", 1234);
        reg.add(&address, "NariFeco")
            .expect("could not add NariFeco");

        let result = reg.add(&address, "CsiterBela");
        let err = result.expect_err("CsiterBela added?");
        assert!(matches!(err, AddError::Unauthorized));
    }

    #[test]
    fn test_register_client_above_slot_limit_should_return_error() {
        let mut reg = ClientRegistry::new(2);
        reg.add(&make_addr("8.8.8.8", 1234), "NariFeco")
            .expect("could not add NariFeco");
        reg.add(&make_addr("1.1.1.1", 1234), "BananJoe")
            .expect("could not add BananJoe");

        let err = reg
            .add(&make_addr("2.2.2.2", 1234), "Overdose")
            .expect_err("Overdose added?");
        assert!(matches!(err, AddError::ServerIsFull));
    }

    #[test]
    fn test_register_different_address_with_same_name_should_return_error() {
        let mut reg = ClientRegistry::new(2);
        reg.add(&make_addr("8.8.8.8", 1234), "NariFeco")
            .expect("could not add NariFeco");
        let err = reg
            .add(&make_addr("1.1.1.1", 1234), "NariFeco")
            .expect_err("could added another NariFeco?");

        assert!(matches!(err, AddError::Unauthorized));
    }

    #[test]
    fn test_remove_by_address_should_remove_it() {
        let mut reg = ClientRegistry::new(2);
        let addr = make_addr("8.8.8.8", 1234);
        let id = reg.add(&addr, "NariFeco").expect("could not add NariFeco");

        let removed_id = reg
            .remove(&addr)
            .expect("Could not remove client by address");
        assert!(reg.find_client(id).is_none());
        assert_eq!(removed_id, id);
    }

    #[test]
    fn test_removing_nonregistered_client_returns_error() {
        let mut reg = ClientRegistry::new(2);
        let addr = make_addr("8.8.8.8", 1234);
        let err = reg
            .remove(&addr)
            .expect_err("Successfully removed a non-registered client???");

        assert!(matches!(err, RemoveError::NoSuchClient));
    }
}
