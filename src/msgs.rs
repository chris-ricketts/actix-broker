use actix::prelude::*;

pub trait BrokerMsg: Message + Send + Clone + 'static {}

impl<M> BrokerMsg for M
where
    M: Message + Send + Clone + 'static,
{
}

#[derive(Message)]
pub struct SubscribeAsync<M: BrokerMsg>(pub Recipient<M>)
where
    <M as Message>::Result: Send;

pub struct SubscribeSync<M: BrokerMsg>(pub Recipient<M>)
where
    <M as Message>::Result: Send;

impl<M: BrokerMsg> Message for SubscribeSync<M>
where
    <M as Message>::Result: Send,
{
    type Result = Option<M>;
}

#[derive(Message)]
pub struct IssueAsync<M: BrokerMsg>(pub M);

#[derive(Message)]
pub struct IssueSync<M: BrokerMsg>(pub M);
