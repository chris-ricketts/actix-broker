//! # Actix-Broker
//!
//! The `actix_broker` crate contains `SystemService` and `ArbiterService` Brokers that 
//! keep track of active subscriptions to different `Messages`. Broker services are 
//! automatically started when an actor uses functions from the `BrokerSubscribe` and 
//! `BrokerIssue` traits to either subscribe to or issue a message.
//!
//! ## Example
//! ```rust,no_run
//! # #[macro_use]
//! # extern crate actix;
//! # extern crate actix_broker;
//! use actix::prelude::*;
//! // Bring both these traits into scope.
//! use actix_broker::{BrokerSubscribe, BrokerIssue, SystemBroker, ArbiterBroker};
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
//!         // Asynchronously subscribe to a message on the system (global) broker
//!         self.subscribe_async::<SystemBroker, MessageOne>(ctx);
//!         // Asynchronously issue a message to any subscribers on the system (global) broker
//!         self.issue_async::<SystemBroker, _>(MessageOne);
//!         // Synchronously subscribe to a message on the arbiter (local) broker
//!         self.subscribe_sync::<ArbiterBroker, MessageTwo>(ctx);
//!         // Synchronously issue a message to any subscribers on the arbiter (local) broker
//!         self.issue_sync::<ArbiterBroker, _>(MessageTwo, ctx);
//!     }
//! }
//!     
//!     // To subscribe to a messsage, the actor must handle it
//! impl Handler<MessageOne> for ActorOne {
//!     type Result = ();
//!
//!     fn handle(&mut self, msg: MessageOne, ctx: &mut Self::Context) {
//!         // An actor does not have to handle a message to just issue it
//!         self.issue_async::<SystemBroker, _>(MessageThree);
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
mod broker;
mod issue;
mod msgs;
mod subscribe;

pub use crate::msgs::BrokerMsg;

pub use crate::broker::{Broker, SystemBroker, ArbiterBroker};

pub use crate::subscribe::BrokerSubscribe;

pub use crate::issue::BrokerIssue;
