extern crate actix;
extern crate actix_broker;

use actix::prelude::*;
use actix_broker::{BrokerIssue, BrokerSubscribe};

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

    fn handle(&mut self, msg: MessageTwo, _ctx: &mut Self::Context) {
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

    fn handle(&mut self, msg: MessageOne, _ctx: &mut Self::Context) {
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

    fn handle(&mut self, msg: MessageOne, _ctx: &mut Self::Context) {
        println!("ActorThree Received: {:?}", msg);
        self.issue_async(MessageTwo(1));
    }
}

#[derive(Clone, Debug, Message)]
struct MessageOne(String);

#[derive(Clone, Debug, Message)]
struct MessageTwo(u8);

fn main() {
    let _ = System::run(|| {
        ActorTwo.start();
        ActorThree.start();
        ActorOne.start();
    });
}
