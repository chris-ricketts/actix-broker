//! # Actix-Broker
//!
//! The `actix_broker` crate contains a Broker `SystemService` that keeps track of active
//! subscriptions to different `Messages`. This Broker service is automatically started
//! when an actor uses functions from the `BrokerSubscribe` and `BrokerIssue` traits to
//! either subscribe to or issue a message.
//!
//! ## Example
//! ```rust,no_run
//! # #[macro_use]
//! # extern crate actix;
//! # extern crate actix_broker;
//! use actix::prelude::*;
//! // Bring both these traits into scope.
//! use actix_broker::{BrokerSubscribe, BrokerIssue};
//!
//! // Note: The message must implement 'Clone'
//! #[derive(Clone, Message)]
//! struct MessageOne;
//!
//! #[derive(Clone, Message)]
//! struct MessageTwo;
//!
//! #[derive(Clone, Message)]
//! struct MessageThree;
//!
//! struct ActorOne;
//!
//! impl Actor for ActorOne {
//!     // Note: The actor context must be Asynchronous,
//!     // i.e. it cannot be 'SyncContext'
//!     type Context = Context<Self>;
//!
//!     fn started(&mut self,ctx: &mut Self::Context) {
//!         // Asynchronously subscribe to a message
//!         self.subscribe_async::<MessageOne>(ctx);
//!         // Asynchronously issue a message to any subscribers
//!         self.issue_async(MessageOne);
//!         // Synchronously subscribe to a message
//!         self.subscribe_sync::<MessageTwo>(ctx);
//!         // Synchronously issue a message to any subscribers
//!         self.issue_sync(MessageTwo, ctx);
//!     }
//! }
//!     
//!     // To subscribe to a messsage, the actor must handle it
//! impl Handler<MessageOne> for ActorOne {
//!     type Result = ();
//!
//!     fn handle(&mut self, msg: MessageOne, ctx: &mut Self::Context) {
//!         // An actor does not have to handle a message to just issue it
//!         self.issue_async(MessageThree);
//!     }
//! }
//!
//!     // Handler for MessageTwo...
//! # impl Handler<MessageTwo> for ActorOne {
//! #     type Result = ();
//!
//! #     fn handle(&mut self, msg: MessageTwo, ctx: &mut Self::Context) {
//! #     }
//! # }
//! # fn main() {}
//! ```
#[macro_use]
extern crate actix;
extern crate fnv;
#[macro_use]
extern crate log;

mod broker;
mod issue;
mod msgs;
mod subscribe;

pub use msgs::BrokerMsg;

pub use broker::Broker;

pub use subscribe::BrokerSubscribe;

pub use issue::BrokerIssue;
