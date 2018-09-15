#[macro_use]
extern crate actix;
extern crate actix_broker;

use actix::prelude::*;
use actix_broker::{BrokerSubscribe, BrokerIssue};

use std::time::Duration;

#[derive(Clone, Message)]
struct TestMessageOne(u8);

#[derive(Clone, Message)]
struct TestMessageTwo(&'static str);

struct TestActor;

impl Actor for TestActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.issue_sync(TestMessageTwo("test"), ctx);
        self.subscribe_async::<TestMessageOne>(ctx);
        ctx.run_later(
            Duration::from_millis(250), 
            |a, _| a.issue_async(TestMessageOne(255))); 
    }
}

impl Handler<TestMessageOne> for TestActor {
    type Result = ();

    fn handle(&mut self, msg: TestMessageOne, ctx: &mut Self::Context) {
        assert_eq!(msg.0, 255);
        self.subscribe_sync::<TestMessageTwo>(ctx);
    }
}

impl Handler<TestMessageTwo> for TestActor {
    type Result = ();

    fn handle(&mut self, msg: TestMessageTwo, _ctx: &mut Self::Context) {
        assert_eq!(msg.0, "test");
        System::current().stop();
    }
}

#[test]
fn it_all_works() {
    System::run(|| { TestActor.start(); });
}
