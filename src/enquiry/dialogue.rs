use std::ops::{Deref, DerefMut};
use tokio::sync::oneshot;
use crate::enquiry::error::AnswerError;

/// Dialogue has a message and provides a way to answer such message.
/// To access the inner message you can either Deref Dialogue
#[derive(Debug)]
pub struct Dialogue<Q, A> {
    message: Q,
    answer_sender: AnswerSender<A>
}

/// AnswerSender can be used to answer a question.
/// If either you only require to borrow the message or message implements Copy, you won't have to
/// interact with this datatype as you can answer a question by calling Dialogue::answer
/// Otherwise, if you require ownership of the message you may want to call Dialogue::into_parts
/// and answer the question by directly using AnswerSender
#[derive(Debug)]
pub struct AnswerSender<A>(oneshot::Sender<A>);

impl<Q, A> Dialogue<Q,A> {
    pub(super) fn new(message: Q, answer_sender: oneshot::Sender<A>) -> Self {
        Self {
            message,
            answer_sender: AnswerSender(answer_sender)
        }
    }
    /// Consumes the Dialogue and gives ownership of the inner message and AnswerSender
    pub fn into_parts(self) -> (Q, AnswerSender<A>) {
        (self.message, self.answer_sender)
    }
    /// Consumes the Dialogue and answers the question.
    pub fn answer(self, answer: A) -> Result<(), AnswerError<A>> {
        self.answer_sender.answer(answer)
    }
}

impl<A> AnswerSender<A> {
    /// Consumes the AnswerSender and answers the question.
    pub fn answer(self, answer: A) -> Result<(), AnswerError<A>> {
        self.0.send(answer)
            .map_err(AnswerError::new)
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