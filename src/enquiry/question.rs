use crate::enquiry::dialogue::Dialogue;
use crate::enquiry::error::{AskError, DialogueError, HearError};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Clone)]
pub struct Question<Q, A> {
    question_sender: mpsc::Sender<Dialogue<Q,A>>,
}

#[derive(Debug)]
pub struct QuestionAsked<A> {
    answer_receiver: oneshot::Receiver<A>
}

impl<Q, A> Question<Q,A> {
    pub(super) fn new(question_sender: mpsc::Sender<Dialogue<Q, A>>) -> Self {
        Self {
            question_sender
        }
    }
    pub async fn ask(self, message: Q) -> Result<QuestionAsked<A>, AskError<Q>> {
        let (answer_sender, answer_receiver) = tokio::sync::oneshot::channel();
        let dialogue = Dialogue::new(message, answer_sender);
        self.question_sender.send(dialogue).await
            .map_err(|e| AskError::new(e.0.into_parts().0))?;
        Ok(
            QuestionAsked {
                answer_receiver,
            }
        )
    }
    pub async fn ask_and_hear(self, message: Q) -> Result<A, DialogueError<Q>> {
        self.ask(message).await
            .map_err(DialogueError::Ask)?
            .hear().await
            .map_err(|_| DialogueError::Hear(HearError(())))
    }
}

impl<A> QuestionAsked<A> {
    pub async fn hear(self) -> Result<A, HearError> {
        self.answer_receiver.await
            .map_err(|_| HearError(()))
    }
}