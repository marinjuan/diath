pub mod question;
pub mod answer;
mod dialogue;
mod error;

#[cfg(test)]
mod tests;

use question::Question;
use answer::Answer;

//@TODO IMPLEMENT VERSION WITH TIMEOUT using tokio_select!

pub fn new<Q, A>(size: usize) -> (Question<Q, A>, Answer<Q, A>) {
    let (question_sender, question_receiver) = tokio::sync::mpsc::channel(size);
    let question = Question::new(question_sender);
    let answer = Answer::new(question_receiver);
    (question, answer)
}
