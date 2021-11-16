use std::fmt::{Debug, Display, Formatter};

/// The Responder side disconnected and we couldn't ask the question
#[derive(Debug)]
pub struct AskError<Q> {
    question: Q,
}

/// The Questioner side disconnected and we couldn't answer the question
#[derive(Debug)]
pub struct Responderror<A> {
    answer: A,
}

/// The Responder side disconnected after the question was asked but before sending an answer
#[derive(Copy, Clone, Debug)]
pub struct ListenError(pub(super)());

/// Either the Responder side disconnected after the question was asked but before sending an answer
/// or timed out
#[derive(Copy, Clone, Debug)]
pub enum ListenTimeoutError {
    Disconnected,
    Timeout,
}

/// Either an AskError or a ListenError
#[derive(Debug)]
pub enum DialogueError<Q> {
    Ask(AskError<Q>),
    Listen,
}

/// Either an AskError, a ListenError or a Timeout
#[derive(Debug)]
pub enum DialogueTimeoutError<Q> {
    Ask(AskError<Q>),
    Listen,
    Timeout,
}

impl<Q> From<DialogueError<Q>> for DialogueTimeoutError<Q> {
    fn from(v: DialogueError<Q>) -> Self {
        match v {
            DialogueError::Listen => DialogueTimeoutError::Listen,
            DialogueError::Ask(v) => DialogueTimeoutError::Ask(v),
        }
    }
}

impl<Q> From<ListenTimeoutError> for DialogueTimeoutError<Q> {
    fn from(v: ListenTimeoutError) -> Self {
        match v {
            ListenTimeoutError::Timeout => DialogueTimeoutError::Timeout,
            ListenTimeoutError::Disconnected => DialogueTimeoutError::Listen,
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

impl<A> Responderror<A> {
    pub(super) fn new(answer: A) -> Self {
        Self {
            answer
        }
    }
    pub fn into_inner(self) -> A {
        self.answer
    }
}

impl<Q> Display for AskError<Q> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt("Responder disconnected before question was asked", f)
    }
}
impl<Q> Display for Responderror<Q> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt("Questioner disconnected before answer was sent", f)
    }
}
impl Display for ListenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt("Responder disconnected before sending an answer", f)
    }
}
impl Display for ListenTimeoutError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ListenTimeoutError::Timeout => Display::fmt("Timeout before answer received", f),
            ListenTimeoutError::Disconnected => Display::fmt(&ListenError(()), f),
        }
    }
}
impl<Q> Display for DialogueError<Q> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogueError::Ask(a) => Display::fmt(a, f),
            DialogueError::Listen => Display::fmt(&ListenError(()), f),
        }
    }
}
impl<Q> Display for DialogueTimeoutError<Q> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogueTimeoutError::Ask(a) => Display::fmt(a, f),
            DialogueTimeoutError::Listen => Display::fmt(&ListenError(()), f),
            DialogueTimeoutError::Timeout => Display::fmt(&ListenTimeoutError::Timeout, f),
        }
    }
}