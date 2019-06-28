extern crate actix;
extern crate actix_broker;
extern crate actix_web;

use actix::prelude::*;
use actix_broker::{Broker, BrokerSubscribe, SystemBroker};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};

#[derive(Clone, Debug, Message)]
struct Hello;

struct TestActor;

impl Actor for TestActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<SystemBroker, Hello>(ctx);
    }
}

impl Handler<Hello> for TestActor {
    type Result = ();

    fn handle(&mut self, msg: Hello, _ctx: &mut Self::Context) {
        println!("TestActor: Received {:?}", msg);
    }
}

fn index(_req: HttpRequest) -> Result<HttpResponse, Error> {
    Broker::<SystemBroker>::issue_async(Hello);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome!"))
}

fn main() {
    let _ = System::run(|| {
        TestActor.start();

        HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(index))))
            .bind("127.0.0.1:8080")
            .unwrap()
            .start();
        println!("Hit up 127.0.0.1:8080");
    });
}
