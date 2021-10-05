use crate::enquiry::answer::Answer;
use crate::enquiry::question::Question;

#[test]
fn test1() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let (question, answer) = super::new(3);
            let q1 = tokio::task::spawn(interrogator(question.clone(), 1));
            let q2 = tokio::task::spawn(eager_interrogator(question.clone(), 2));
            let q3 = tokio::task::spawn(interrogator(question, 3));
            let end = tokio::task::spawn(suspect(answer));
            assert!(q1.await.unwrap());
            assert!(q2.await.unwrap());
            assert!(q3.await.unwrap());
            assert!(end.await.unwrap());
        })
}

async fn interrogator(question: Question<u8, u16>, v: u8) -> bool {
    let question = question.ask(v).await.unwrap();
    question.hear().await.unwrap() == (v as u16) * 2
}

async fn eager_interrogator(question: Question<u8, u16>, v: u8) -> bool {
    question.ask_and_hear(v).await.unwrap() == (v as u16) * 2
}

async fn suspect(mut answer: Answer<u8, u16>) -> bool {
    while let Some(question) = answer.hear().await {
        let value = *question as u16;
        question.answer(value * 2).unwrap();
    }
    true
}
