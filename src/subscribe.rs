//! messages.
use actix::dev::ToEnvelope;
use actix::prelude::*;

use std::any::TypeId;

use crate::broker::RegisteredBroker;
use crate::msgs::*;

/// The `BrokerSubscribe` trait has functions to register an actor's interest in different
/// messages.
pub trait BrokerSubscribe
where
    Self: Actor,
    <Self as Actor>::Context: AsyncContext<Self>,
{
    /// Asynchronously subscribe to a message.
    fn subscribe_async<T: RegisteredBroker,  M: BrokerMsg>(&self, ctx: &mut Self::Context)
    where
        Self: Handler<M>,
        <Self as Actor>::Context: ToEnvelope<Self, M>,
    {
        let broker = T::get_broker();
        let recipient = ctx.address().recipient::<M>();
        broker.do_send(SubscribeAsync(recipient, TypeId::of::<Self>()));
    }

    /// Synchronously subscribe to a message.
    /// This actor will do nothing else until its interest is registered.
    /// If messages of that type have been sent to the broker previously, a copy of the latest
    /// message is sent to the calling actor after it has subscribed.
    fn subscribe_sync<T: RegisteredBroker, M: BrokerMsg>(&self, ctx: &mut Self::Context)
    where
        Self: Handler<M>,
        <Self as Actor>::Context: ToEnvelope<Self, M>,
    {
        let broker = T::get_broker();
        let recipient = ctx.address().recipient::<M>();

        broker
            .send(SubscribeSync(recipient, TypeId::of::<Self>()))
            .into_actor(self)
            .map_err(|_, _, _| ())
            .map(move |m, _, ctx| {
                if let Some(msg) = m {
                    ctx.notify(msg);
                }
            })
            .wait(ctx);
    }
}

impl<A> BrokerSubscribe for A
where
    A: Actor,
    <A as Actor>::Context: AsyncContext<A>,
{
}
