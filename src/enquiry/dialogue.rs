use std::ops::{Deref, DerefMut};
use tokio::sync::oneshot;
use crate::enquiry::error::Responderror;

/// Dialogue has a message and provides a way to answer such message.
/// To access the inner message you can either Deref Dialogue
#[derive(Debug)]
pub struct Dialogue<Q, A> {
    message: Q,
    answer_sender: ResponseChannel<A>
}

/// ResponseChannel can be used to answer a question.
/// If either you only require to borrow the message or message implements Copy, you won't have to
/// interact with this datatype as you can answer a question by calling Dialogue::answer
/// Otherwise, if you require ownership of the message you may want to call Dialogue::into_parts
/// and answer the question by directly using ResponseChannel
#[derive(Debug)]
pub struct ResponseChannel<A>(oneshot::Sender<A>);

impl<Q, A> Dialogue<Q,A> {
    /// Creates a new Dialogue
    pub(super) fn new(message: Q, answer_sender: oneshot::Sender<A>) -> Self {
        Self {
            message,
            answer_sender: ResponseChannel(answer_sender)
        }
    }
    /// Consumes the Dialogue and gives ownership of the inner message and ResponseChannel
    pub fn into_parts(self) -> (Q, ResponseChannel<A>) {
        (self.message, self.answer_sender)
    }
    /// Consumes the Dialogue and answers the question.
    pub fn answer(self, answer: A) -> Result<(), Responderror<A>> {
        self.answer_sender.answer(answer)
    }
}

impl<A> ResponseChannel<A> {
    /// Consumes the ResponseChannel and answers the question.
    pub fn answer(self, answer: A) -> Result<(), Responderror<A>> {
        self.0.send(answer)
            .map_err(Responderror::new)
    }
}

impl<Q, A> Deref for Dialogue<Q, A> {
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl<Q, A> DerefMut for Dialogue<Q, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.message
    }
}