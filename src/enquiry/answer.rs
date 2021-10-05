use crate::enquiry::dialogue::Dialogue;
use tokio::sync::mpsc;

pub struct Answer<Q, A> {
    question_receiver: mpsc::Receiver<Dialogue<Q, A>>,
}

impl<Q, A> Answer<Q, A> {
    pub(super) fn new(question_receiver: mpsc::Receiver<Dialogue<Q, A>>) -> Self {
        Self {
            question_receiver
        }
    }
    pub async fn hear(&mut self) -> Option<Dialogue<Q, A>> {
        self.question_receiver.recv().await
    }
}