# Event emitting issue/usage

Either the usage of the event/emit functionality is wrong, or there is a possible bug.
The has 2 components: an event listener that runs off chain, and an event producer script

### Install Submodules

#### Build/deploy the anchor producer - need validator running

```
cd events-test-producer
anchor deploy
anchor idl parse --file programs/events-test/src/lib.rs  > idl.json
```
#### copy the Deployed PID into the main.rs of the listner
```
cd ..
cd events-test-listener
cargo run
```

#### run the script to produce events
```
cd ..
cd events-test-producer
node run.js
```

#### observe the following:

```
JsonParseError(Error("EOF while parsing a value", line: 1, column: 0))
websocket - exited receive loop
```

### NOTE
the error is not observed if OrdStatus enum is not present


