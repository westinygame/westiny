![Westiny logo](media/westiny_logo.png)
# westiny
This is a top-down sandbox game.  
The game is written fully in rust.

## usage

### server
Run:
`cargo run --release --bin westiny_server`

### client
Specify server address on client:
`export WESTINY_SERVER_ADDRESS=1.2.3.4:5745`

Run:
`cargo run --release --bin westiny`

Or a one-liner:
`WESTINY_SERVER_ADDRESS=1.2.3.4:5745 cargo run --release --bin westiny`

