# Pingora Project

This project demonstrates the use of Pingora for implementing various networking features such as a reverse proxy, load balancer, and health check mechanisms. Pingora is a powerful and flexible library that enables efficient handling of network connections within Rust applications.

## Features

- **Reverse Proxy**: Forward requests to another server, allowing for functionalities like load balancing, caching, or layer 7 routing.
- **Load Balancer**: Distribute incoming network traffic across multiple servers to ensure no single server bears too much demand.
- **Health Check**: Monitor the status of upstream servers to ensure traffic is only directed to healthy instances.

## Getting Started

To run the examples, make sure you have Rust and Cargo installed on your system. Clone this repository, navigate to the project directory, and build/run the project using Cargo.

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Running the Reverse Proxy Example

This example demonstrates setting up a simple reverse proxy that forwards requests to a local application running on `localhost:3000`.

```bash
cargo run
```

Use app by visiting: <http://127.0.0.1:3000>
