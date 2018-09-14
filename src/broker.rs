use actix::prelude::*;
use fnv::FnvHasher;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::mem;

use msgs::*;

type TypeMap<A> = HashMap<TypeId, A, BuildHasherDefault<FnvHasher>>;

#[derive(Default)]
pub struct Broker {
    sub_map: TypeMap<Vec<Box<Any>>>,
    msg_map: TypeMap<Box<Any>>,
}

impl Broker {
    fn take_subs<M: BrokerMsg>(&mut self) -> Option<Vec<Recipient<M>>>
    where
        <M as Message>::Result: Send,
    {
        let id = TypeId::of::<M>();
        let subs = self.sub_map.get_mut(&id)?;
        trace!("Broker: Found subscription list for {:?}.", id);
        let mut subs = mem::replace(subs, Vec::new());
        let subs = subs.drain(..)
            .filter_map(|s| s.downcast::<Recipient<M>>().ok())
            .map(|s| *s)
            .collect();
        Some(subs)
    }

    fn add_sub<M: BrokerMsg>(&mut self, sub: Recipient<M>)
    where
        <M as Message>::Result: Send,
    {
        let id = TypeId::of::<M>();
        let boxed = Box::new(sub);
        if let Some(subs) = self.sub_map.get_mut(&id) {
            trace!("Broker: Adding to {:?} subscription list.", id);
            subs.push(boxed);
            return;
        }

        trace!("Broker: Creating {:?} subscription list.", id);
        self.sub_map.insert(id, vec![boxed]);
    }

    fn get_previous_msg<M: BrokerMsg>(&self) -> Option<M> {
        let id = TypeId::of::<M>();
        let msg = self.msg_map.get(&id)?;
        trace!("Broker: Previous message found for {:?}", id);
        let msg = msg.downcast_ref::<M>()?;
        Some(msg.clone())
    }

    fn set_msg<M: BrokerMsg>(&mut self, msg: M) {
        let id = TypeId::of::<M>();
        let boxed = Box::new(msg);
        if let Some(pm) = self.msg_map.get_mut(&id) {
            trace!("Broker: Setting new message value for {:?}", id);
            mem::replace(pm, boxed);
            return;
        }

        trace!("Broker: Adding first message value for {:?}", id);
        self.msg_map.insert(id, boxed);
    }
}

impl<M: BrokerMsg> Handler<SubscribeAsync<M>> for Broker
where
    <M as Message>::Result: Send,
{
    type Result = ();

    fn handle(&mut self, msg: SubscribeAsync<M>, _ctx: &mut Context<Self>) {
        trace!("Broker: Received SubscribeAsync");
        self.add_sub::<M>(msg.0);
    }
}

impl<M: BrokerMsg> Handler<SubscribeSync<M>> for Broker
where
    <M as Message>::Result: Send,
{
    type Result = Option<M>;

    fn handle(&mut self, msg: SubscribeSync<M>, _ctx: &mut Context<Self>) -> Self::Result {
        trace!("Broker: Received SubscribeSync");
        self.add_sub::<M>(msg.0);
        self.get_previous_msg::<M>()
    }
}

impl<M: BrokerMsg> Handler<IssueAsync<M>> for Broker
where
    <M as Message>::Result: Send,
{
    type Result = ();

    fn handle(&mut self, msg: IssueAsync<M>, _ctx: &mut Context<Self>) {
        trace!("Broker: Received IssueAsync");
        if let Some(mut subs) = self.take_subs::<M>() {
            subs.drain(..)
                .filter_map(|s| {
                    if s.do_send(msg.0.clone()).is_ok() {
                        Some(s)
                    } else {
                        None
                    }
                })
                .for_each(|s| self.add_sub::<M>(s));
        }
        self.set_msg::<M>(msg.0);
    }
}

impl<M: BrokerMsg> Handler<IssueSync<M>> for Broker
where
    <M as Message>::Result: Send,
{
    type Result = ();

    fn handle(&mut self, msg: IssueSync<M>, ctx: &mut Context<Self>) {
        trace!("Broker: Received IssueSync");
        if let Some(mut subs) = self.take_subs::<M>() {
            subs.drain(..).for_each(|s| {
                s.send(msg.0.clone())
                    .into_actor(self)
                    .map_err(|_, _, _| ())
                    .map(move |_, act, _| act.add_sub::<M>(s))
                    .wait(ctx);
            });
        }
        self.set_msg::<M>(msg.0);
    }
}

impl Actor for Broker {
    type Context = Context<Self>;
}

impl SystemService for Broker {}

impl Supervised for Broker {}
