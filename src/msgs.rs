use actix::prelude::*;

use std::any::TypeId;

pub trait BrokerMsg: Message<Result = ()> + Send + Clone + 'static {}

impl<M> BrokerMsg for M
where
    M: Message<Result = ()> + Send + Clone + 'static,
{
}

#[derive(Message)]
pub struct SubscribeAsync<M: BrokerMsg>(pub Recipient<M>, pub TypeId);

pub struct SubscribeSync<M: BrokerMsg>(pub Recipient<M>, pub TypeId);

impl<M: BrokerMsg> Message for SubscribeSync<M> {
    type Result = Option<M>;
}

#[derive(Message)]
pub struct IssueAsync<M: BrokerMsg>(pub M, pub TypeId);

#[derive(Message)]
pub struct IssueSync<M: BrokerMsg>(pub M, pub TypeId);
