pub mod question;
pub mod answer;
mod dialogue;
#[cfg(test)]
mod tests;
mod error;

use question::Question;
use answer::Answer;

//@TODO TIENEN QUE MANEJAR UN STRUCT MESSAGE que se pasan, este struct tiene el mensaje interno y el oneshot para contestar
//@TODO ADEMAS implementa deref? para poder usar los metodos del subtipo interno directamente
//@TODO ADEMAS tiene into_parts que devuelve la tupla oneshot, value
//@TODO ADEMAS se le puede sacar su value con un take_message
//@TODO LUEGO IMPLEMENTAR VERSION CON TIMEOUT usando tokio_select!, este debe consumir el sender!

pub fn new<Q, A>(size: usize) -> (Question<Q, A>, Answer<Q, A>) {
    let (question_sender, question_receiver) = tokio::sync::mpsc::channel(size);
    let question = Question::new(question_sender);
    let answer = Answer::new(question_receiver);
    (question, answer)
}
