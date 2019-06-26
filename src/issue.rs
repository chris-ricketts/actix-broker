use actix::prelude::*;

use std::any::TypeId;

use crate::broker::{RegisteredBroker};
use crate::msgs::*;

/// The `BrokerIssue` provides functions to issue messages to any subscribers.
pub trait BrokerIssue
where
    Self: Actor,
    <Self as Actor>::Context: AsyncContext<Self>,
{
    /// Asynchronously issue a message.
    fn issue_async<T: RegisteredBroker, M: BrokerMsg>(&self, msg: M) {
        let broker = T::get_broker();
        broker.do_send(IssueAsync(msg, TypeId::of::<Self>()));
    }

    /// Synchronously issue a message.
    /// This also causes the broker to synchronously forward those messages on to any subscribers
    /// before handling any other messages.
    fn issue_sync<T: RegisteredBroker, M: BrokerMsg>(&self, msg: M, ctx: &mut Self::Context) {
        let broker = T::get_broker();
        broker
            .send(IssueSync(msg, TypeId::of::<Self>()))
            .into_actor(self)
            .map_err(|_, _, _| ())
            .map(|_, _, _| ())
            .wait(ctx);
    }
}

impl<A> BrokerIssue for A
where
    A: Actor,
    <A as Actor>::Context: AsyncContext<A>,
{
}
