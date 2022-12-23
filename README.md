# wol-http-server

## About wol-http-server

Wol-http-server is a simple service written in Rust that allows you to wake up devices on your local network (LAN) using the Wake-on-LAN (WOL) protocol. With wol-http-server, you can send a WOL packet to a device by making a simple HTTP POST request to the server.

## Usage

When the service runs somewhere and the device you want to target has WOL enabled you'll need to make a `POST` request to `http://<ip/host>:<port>/wake?mac=<mac>`

Where:

- `<ip/host>` should contain the (local) IP or hostname where the service is running on e.g. 192.168.0.0
- `<port>` should contain the same port you also passed the program as argument
- `<mac>` should contain the MAC address of the device you are targeting e.g. `00:00:00:00:00:00`

Curl example: `curl -X POST "http://192.168.0.0:0000/wake?mac=00:00:00:00:00:00"`

If you want to test if you can reach the service, it also provides a ping GET endpoint: `http://<ip/host>:<port>/ping`

Curl example: `curl -X GET "http://192.168.0.00:0000/ping"`

## Building the program

### Prerequisites

- Rust compiler and tools (installation instructions can be found at https://www.rust-lang.org/tools/install)

## Build

1. Clone this repo
2. Open a terminal in the project
3. Run `cargo build --release` (Optionally compile program for a different target: https://rust-lang.github.io/rustup/cross-compilation.html)
4. After building you'll find the executable at `target/release/wol-http-server`

## Running the program

### \*nix operating systems

1. Run `./wol-http-server <port>` where `<port>` will be replaced by the port you want to run the service on
