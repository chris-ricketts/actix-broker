extern crate actix;
extern crate actix_broker;
extern crate actix_derive;
extern crate simple_logger;
#[macro_use]
extern crate log;

use actix::prelude::*;
use actix_broker::{BrokerIssue, BrokerSubscribe};

#[derive(Default)]
struct ActorOne;

#[derive(Default)]
struct ActorTwo;

#[derive(Default, Debug, Clone, Message)]
struct MessageOne(u8);

impl Actor for ActorOne {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<MessageOne>(ctx);
    }
}

impl Handler<MessageOne> for ActorOne {
    type Result = ();

    fn handle(&mut self, msg: MessageOne, ctx: &mut Self::Context) {
        info!("Received: {:?}", msg);
        System::current().stop();
    }
}

impl Actor for ActorTwo {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.issue_sync::<MessageOne>(MessageOne(9), ctx);
    }
}

#[test]
fn it_works() {
    simple_logger::init().unwrap();
    System::run(|| {
        ActorOne::start_default();
        ActorTwo::start_default();
    });
}
