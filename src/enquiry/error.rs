/// The Answerer side disconnected and we couldn't ask the question
#[derive(Debug)]
pub struct AskError<Q> {
    question: Q,
}

/// The Questioner side disconnected and we couldn't answer the question
#[derive(Debug)]
pub struct AnswerError<A> {
    answer: A,
}

/// The Answerer side disconnected after the question was asked but before sending an answer
#[derive(Copy, Clone, Debug)]
pub struct HearError(pub(super)());

/// Either the Answerer side disconnected after the question was asked but before sending an answer
/// or timed out
#[derive(Copy, Clone, Debug)]
pub enum HearTimeoutError {
    Disconnected,
    Timeout,
}

/// Either an AskError or a HearError
#[derive(Debug)]
pub enum DialogueError<Q> {
    Ask(AskError<Q>),
    Hear(HearError),
}

impl<Q> AskError<Q> {
    pub(super) fn new(question: Q) -> Self {
        Self {
            question
        }
    }
}

impl<A> AnswerError<A> {
    pub(super) fn new(answer: A) -> Self {
        Self {
            answer
        }
    }
}