use actix::prelude::*;

use broker::Broker;
use msgs::*;

pub trait BrokerIssue: Actor<Context = Context<Self>> {
    fn issue_async<M: BrokerMsg>(&self, msg: M)
    where
        <M as Message>::Result: Send,
    {
        let broker = Broker::from_registry();
        broker.do_send(IssueAsync(msg));
    }

    fn issue_sync<M: BrokerMsg>(&self, msg: M, ctx: &mut Self::Context)
    where
        <M as Message>::Result: Send,
    {
        let broker = Broker::from_registry();
        broker
            .send(IssueSync(msg))
            .into_actor(self)
            .map_err(|_, _, _| ())
            .map(|_, _, _| ())
            .wait(ctx);
    }
}

impl<A> BrokerIssue for A
where
    A: Actor<Context = Context<A>>,
{
}
