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
    Hear,
}

/// Either an AskError, a HearError or a Timeout
#[derive(Debug)]
pub enum DialogueTimeoutError<Q> {
    Ask(AskError<Q>),
    Hear,
    Timeout,
}

impl<Q> From<DialogueError<Q>> for DialogueTimeoutError<Q> {
    fn from(v: DialogueError<Q>) -> Self {
        match v {
            DialogueError::Hear => DialogueTimeoutError::Hear,
            DialogueError::Ask(v) => DialogueTimeoutError::Ask(v),
        }
    }
}

impl<Q> From<HearTimeoutError> for DialogueTimeoutError<Q> {
    fn from(v: HearTimeoutError) -> Self {
        match v {
            HearTimeoutError::Timeout => DialogueTimeoutError::Timeout,
            HearTimeoutError::Disconnected => DialogueTimeoutError::Hear,
        }
    }
}

impl<Q> AskError<Q> {
    pub(super) fn new(question: Q) -> Self {
        Self {
            question
        }
    }
    pub fn into_inner(self) -> Q {
        self.question
    }
}

impl<A> AnswerError<A> {
    pub(super) fn new(answer: A) -> Self {
        Self {
            answer
        }
    }
    pub fn into_inner(self) -> A {
        self.answer
    }
}