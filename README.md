# Actix-Broker
A crate that adds a message broker to the Actix actor framework.

Subscribe to and issue Actix Messages easily in an asynchronous or synchronous manner.

## Usage

Simply include the `BrokerSubscribe` & `BrokerIssue` traits then the following functions can be used:
```rust
// Asynchronously subscribe to a message
self.subscribe_async::<MessageType>(ctx);

// Asynchronoulsy send a message to any subscribers
self.issue_async(MessageType);

// Synchronously subscribe to a message
// The calling actor will be notified of the 
// last message issued to the broker for that MessageType
self.subscribe_sync::<MessageType>(ctx);

// Synchronously send a message to any subscribers.
self.issue_sync(MessageType, ctx);
```

## Example

```rust
#[macro_use]
extern crate actix;
extern crate actix_broker;

use actix::prelude::*;
use actix_broker::{BrokerSubscribe, BrokerIssue};

struct ActorOne;
struct ActorTwo;
struct ActorThree;

impl Actor for ActorOne {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_sync::<MessageTwo>(ctx);
        self.issue_async(MessageOne("hello".to_string()));
    }
}

impl Handler<MessageTwo> for ActorOne {
    type Result = ();

    fn handle(&mut self, msg: MessageTwo, ctx: &mut Self::Context) {
        println!("ActorOne Received: {:?}", msg);
    }
}

impl Actor for ActorTwo {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<MessageOne>(ctx);
    }
}

impl Handler<MessageOne> for ActorTwo {
    type Result = ();

    fn handle(&mut self, msg: MessageOne, ctx: &mut Self::Context) {
        println!("ActorTwo Received: {:?}", msg);
        self.issue_async(MessageTwo(0));
    }
}

impl Actor for ActorThree {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<MessageOne>(ctx);
    }
}

impl Handler<MessageOne> for ActorThree {
    type Result = ();

    fn handle(&mut self, msg: MessageOne, ctx: &mut Self::Context) {
        println!("ActorThree Received: {:?}", msg);
        self.issue_async(MessageTwo(1));
    }
}

#[derive(Clone, Debug, Message)]
struct MessageOne(String);

#[derive(Clone, Debug, Message)]
struct MessageTwo(u8);

fn main() {
    System::run(|| {
        ActorTwo.start();
        ActorThree.start();
        ActorOne.start();
    });
}
```
