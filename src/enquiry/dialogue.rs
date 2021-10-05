use std::ops::Deref;
use tokio::sync::oneshot;
use crate::enquiry::error::AnswerError;

#[derive(Debug)]
pub struct Dialogue<Q, A> {
    message: Q,
    answer_sender: AnswerSender<A>
}

#[derive(Debug)]
pub struct AnswerSender<A>(oneshot::Sender<A>);

impl<Q, A> Dialogue<Q,A> {
    pub(super) fn new(message: Q, answer_sender: oneshot::Sender<A>) -> Self {
        Self {
            message,
            answer_sender: AnswerSender(answer_sender)
        }
    }
    pub fn into_parts(self) -> (Q, AnswerSender<A>) {
        (self.message, self.answer_sender)
    }
    pub fn answer(self, answer: A) -> Result<(), AnswerError<A>> {
        self.answer_sender.answer(answer)
    }
}

impl<A> AnswerSender<A> {
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