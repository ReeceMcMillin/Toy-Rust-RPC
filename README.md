# Overview

An assignment in my distributed computing class required us to use a Python library [(xmlrpc)](https://docs.python.org/3/library/xmlrpc.html) to implement a basic RPC system with a client, a server, and two workers. I wanted to try implementing the project without an RPC library just to get a better understanding of the ideas and issues those systems face. If you stumble across this, just be aware that it probably shouldn't be used as a reference!

# Problems

This was extraneous work alongside a much more straightforward assignment and I didn't want to spend a ton of time on ungraded work, so some things are broken/rushed. It works as expected under the happy path, which involves things like the following:

- Worker nodes must be started before server node
- Worker nodes must be known at server's compile-time
- Worker nodes must be started manually

These would be real problems for a real RPC system, but this isn't one of those!

# Usage

In four separate terminals and in the following order:
1. Start worker 1: `cargo run --bin worker -- --port 23001 --group am`
2. Start worker 2: `cargo run --bin worker -- --port 23002 --group nz`
3. Start server: `cargo run --bin server -- --port 23000`
4. Run client app: `cargo run --bin client -- --port 23000`

The output looks something like the following:

```
[client/src/main.rs:17] proxy.get_by_name("rakin".to_string()).await = {
    "rakin": Person {
        record_id: 1,
        name: "rakin",
        location: "Kansas City",
        year: 2019,
    },
}
[client/src/main.rs:18] proxy.get_by_location(...

...

[client/src/main.rs:19] proxy.get_by_year("Kansas City".to_string(), 2018).await = {
    "zen": Person {
        record_id: 3,
        name: "zen",
        location: "Kansas City",
        year: 2018,
    },
}
```
