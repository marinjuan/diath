use crate::enquiry::dialogue::Dialogue;
use tokio::sync::mpsc;

/// Responder can Listen questions, when a question is asked a new `Dialogue` is created
/// Instances are created by the [`enquiry::new(size)`](crate::enquiry::new()) function
pub struct Responder<Q, A> {
    question_receiver: mpsc::Receiver<Dialogue<Q, A>>,
}

impl<Q, A> Responder<Q, A> {
    pub(super) fn new(question_receiver: mpsc::Receiver<Dialogue<Q, A>>) -> Self {
        Self {
            question_receiver
        }
    }
    /// Listen until a question is asked
    pub async fn listen(&mut self) -> Option<Dialogue<Q, A>> {
        self.question_receiver.recv().await
    }
}