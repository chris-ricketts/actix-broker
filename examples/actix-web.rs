#[macro_use]
extern crate actix;
extern crate actix_web;
extern crate actix_broker;

use actix::prelude::*;
use actix_web::{server, App, Error, HttpRequest, HttpResponse};
use actix_broker::{Broker, BrokerSubscribe};

#[derive(Clone, Debug, Message)] 
struct Hello;

struct TestActor;

impl Actor for TestActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<Hello>(ctx);
    }
}

impl Handler<Hello> for TestActor {
    type Result = ();

    fn handle(&mut self, msg: Hello, _ctx: &mut Self::Context) {
        println!("TestActor: Received {:?}", msg);
    }
}

fn index(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    Broker::issue_async(Hello);
    Ok(HttpResponse::Ok().content_type("text/plain").body("Welcome!"))
}

fn main() {
    System::run(|| {
        TestActor.start();

        server::new(||
            App::new()
                .resource("/", |r| r.f(index)))
            .bind("127.0.0.1:8080")
            .unwrap()
            .start();
        println!("Hit up 127.0.0.1:8080");
    });
}
