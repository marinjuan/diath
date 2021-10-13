use crate::enquiry::dialogue::Dialogue;
use tokio::sync::mpsc;

/// Answer can hear questions, when a question is received a new `Dialogue` is created
/// Instances are created by the [`enquiry::new(size)`](crate::enquiry::new()) function
pub struct Answerer<Q, A> {
    question_receiver: mpsc::Receiver<Dialogue<Q, A>>,
}

impl<Q, A> Answerer<Q, A> {
    pub(super) fn new(question_receiver: mpsc::Receiver<Dialogue<Q, A>>) -> Self {
        Self {
            question_receiver
        }
    }
    /// hear until a question is asked
    pub async fn hear(&mut self) -> Option<Dialogue<Q, A>> {
        self.question_receiver.recv().await
    }
}