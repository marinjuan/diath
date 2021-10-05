#[derive(Debug)]
pub struct AskError<Q> {
    question: Q,
}

#[derive(Debug)]
pub struct AnswerError<A> {
    answer: A,
}

#[derive(Debug)]
pub struct HearError(pub(super)());

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