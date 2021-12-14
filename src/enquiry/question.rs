use std::time::Duration;
use crate::enquiry::dialogue::Dialogue;
use crate::enquiry::error::{AskError, DialogueError, DialogueTimeoutError, ListenError, ListenTimeoutError};
use tokio::sync::{mpsc, oneshot};

/// Questioner can be used to ask questions to the Answer side.
/// Questioner is cheap to Clone, consider cloning it when you will have to ask questions from multiple tasks
/// Instances are created by the [`enquiry::new(size)`](crate::enquiry::new()) function
#[derive(Debug)]
pub struct Questioner<Q, A> {
    question_sender: mpsc::Sender<Dialogue<Q,A>>,
}

/// QuestionAsked can be used to Listen an answer
#[derive(Debug)]
pub struct QuestionAsked<A> {
    answer_receiver: oneshot::Receiver<A>
}

impl<Q, A> Questioner<Q,A> {
    pub(super) fn new(question_sender: mpsc::Sender<Dialogue<Q, A>>) -> Self {
        Self {
            question_sender
        }
    }
    /// Ask a question
    pub async fn ask(&self, message: Q) -> Result<QuestionAsked<A>, AskError<Q>> {
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
    /// Ask a question and also wait for an answer
    pub async fn ask_and_listen(&self, message: Q) -> Result<A, DialogueError<Q>> {
        self.ask(message).await
            .map_err(DialogueError::Ask)?
            .listen().await
            .map_err(|_| DialogueError::Listen)
    }
    /// Ask a question and also wait for an answer
    pub async fn ask_and_listen_or_timeout(&self, message: Q, timeout: impl Into<Duration>) -> Result<A, DialogueTimeoutError<Q>> {
        self.ask(message).await
            .map_err(DialogueTimeoutError::Ask)?
            .listen_or_timeout(timeout).await
            .map_err(|e| e.into())
    }
}

impl<A> QuestionAsked<A> {
    /// Wait until an answer arrives
    pub async fn listen(self) -> Result<A, ListenError> {
        self.answer_receiver.await
            .map_err(|_| ListenError(()))
    }
    /// Wait until an answer arrives or a certain time has elapsed
    pub async fn listen_or_timeout(self, timeout: impl Into<Duration>) -> Result<A, ListenTimeoutError> {
        tokio::select! {
            biased;
            result = self.answer_receiver => {
                result.map_err(|_| ListenTimeoutError::Disconnected)
            },
            _ = tokio::time::sleep(timeout.into()) => {
                Err(ListenTimeoutError::Timeout)
            }
        }
    }
}

impl<Q, A> std::clone::Clone for Questioner<Q, A> {
    fn clone(&self) -> Self {
        Self {
            question_sender: self.question_sender.clone(),
        }
    }
}
