pub mod question;
pub mod answer;
mod dialogue;
mod error;

#[cfg(test)]
mod tests;

use question::Questioner;
use answer::Answerer;

/// Creates a new tuple of (`Question<Q, A>`, `Answer<Q, A>`)
pub fn new<Q, A>(size: usize) -> (Questioner<Q, A>, Answerer<Q, A>) {
    let (question_sender, question_receiver) = tokio::sync::mpsc::channel(size);
    let question = Questioner::new(question_sender);
    let answer = Answerer::new(question_receiver);
    (question, answer)
}
