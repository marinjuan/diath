//! Utility for enquiry between 2 threads. This means that one thread starts the communication by
//! sending data to the other one (ask a question) and the other thread sending data back (answering
//! the question).
//! Under the hood enquiry uses two tokio channels, first an mpsc for asking the question (so that
//! multiple questions can be asked) and second a oneshot for answering (as only one answer per
//! question is allowed)
//! This is intended to be a two way async communication.
//!
//! **Examples:**
//!
//! ```
//! use diath::enquiry;
//! #[tokio::main]
//! async fn main() {
//!     let (q1, q2) = ("Are you innocent?".to_string(), "Did you kill him?".to_string());
//!     let (questioner, answerer) = enquiry::new(3);
//!     let i1 = tokio::task::spawn(interrogator(questioner.clone(), q1));
//!     let i2 = tokio::task::spawn(interrogator(questioner, q2));
//!     tokio::task::spawn(suspect(answer));
//!     assert!(i1.await.unwrap()); // Assert it is innocent
//!     assert!(!i2.await.unwrap()); // Assert it did not kill the victim
//! }
//! async fn interrogator(questioner: enquiry::question::Questioner<String, bool>, question: String) -> bool {
//!     questioner.ask_and_hear_answer(question).await.unwrap()
//! }
//! async fn suspect(mut answerer: enquiry::answer::Answerer<String, bool>) {
//!     while let Some(dialogue) = answerer.hear().await {
//!         match dialogue.as_str() {
//!             "Did you kill him?" => false,
//!             "Are you innocent?" => true,
//!             _ => unreachable!(),
//!         }
//!     }
//! }
//! ```

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
