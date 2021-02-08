use westiny_common::components;

#[derive(Default)]
pub struct NetworkIdSupplier {
    next_id: u64,
}

impl NetworkIdSupplier {
    pub fn new() -> Self {
        NetworkIdSupplier { next_id: 0 }
    }

    pub fn next(&mut self) -> components::NetworkId {
        let id = components::NetworkId::new(self.next_id);
        self.next_id += 1;
        id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn network_id_supplier_get_and_increment() {
        let mut supplier = NetworkIdSupplier::new();
        for i in 0..100_000 {
            assert_eq!(&i, supplier.next().get());
        }
    }
}