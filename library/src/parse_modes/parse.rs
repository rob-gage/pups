// Copyright Rob Gage 2025

use crate::ParseMode;
use std::marker::PhantomData;

pub struct Parse<Error, Message, Output> (PhantomData<(Error, Message, Output)>);

impl<E, M, O> ParseMode for Parse<E, M, O> {

    type Error = E;

    type Message = M;

    type MessageContainer = Vec<M>;

    type Output = O;

    fn add_message_to_container(
        message: Self::Message,
        message_container: &mut Self::MessageContainer
    ) { message_container.push(message); }

    fn combine_message_containers(
        a: &mut Self::MessageContainer,
        b: Self::MessageContainer
    ) { a.extend(b) }

    fn new_message_container() -> Self::MessageContainer { Vec::new() }

}