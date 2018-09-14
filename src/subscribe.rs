use actix::prelude::*;

use broker::Broker;
use msgs::*;

pub trait BrokerSubscribe: Actor<Context = Context<Self>> {
    fn subscribe_async<M: BrokerMsg>(&self, ctx: &mut Self::Context)
    where
        <M as Message>::Result: Send,
        Self: Handler<M>,
    {
        let broker = Broker::from_registry();
        let recipient = ctx.address().recipient::<M>();
        broker.do_send(SubscribeAsync(recipient));
    }

    fn subscribe_sync<M: BrokerMsg>(&self, ctx: &mut Self::Context)
    where
        <M as Message>::Result: Send,
        Self: Handler<M>,
    {
        let broker = Broker::from_registry();
        let recipient = ctx.address().recipient::<M>();

        broker
            .send(SubscribeSync(recipient))
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
    A: Actor<Context = Context<A>>,
{
}
