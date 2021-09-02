![Westiny logo](media/westiny_logo.png)
# westiny
This is a western topview sandbox game.  
The game is written fully in rust.

## usage

### server
Run:
`cargo run --release --bin westiny_server`

By default the server will be listening on `127.0.0.1:5745`.
To modify this address, edit `resources/server_address.ron`.

### client
Specify server address on client:
`export WESTINY_SERVER_ADDRESS=1.2.3.4:5745`

Run:
`cargo run --release --bin westiny_client`

Or a one-liner:
`WESTINY_SERVER_ADDRESS=1.2.3.4:5745 cargo run --release --bin westiny_client`

### running server and client on the same computer
Start the server with default address:
`cargo run --release --bin westiny_server`

Start the client with default address too:
`cargo run --release --bin westiny_client`
Note, that `WESTINY_SERVER_ADDRESS` has not been set.
