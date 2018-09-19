use actix::prelude::*;

pub trait BrokerMsg: Message<Result = ()> + Send + Clone + 'static {}

impl<M> BrokerMsg for M
where
    M: Message<Result = ()> + Send + Clone + 'static,
{
}

#[derive(Message)]
pub struct SubscribeAsync<M: BrokerMsg>(pub Recipient<M>);

pub struct SubscribeSync<M: BrokerMsg>(pub Recipient<M>);

impl<M: BrokerMsg> Message for SubscribeSync<M> {
    type Result = Option<M>;
}

#[derive(Message)]
pub struct IssueAsync<M: BrokerMsg>(pub M);

#[derive(Message)]
pub struct IssueSync<M: BrokerMsg>(pub M);
