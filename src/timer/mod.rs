use std::future::Future;
use tokio::task::JoinHandle;

/// Represents the possible results of running an async task until timeout
pub enum TaskTimeoutResult<Result>
where Result: Send {
    Timeout,
    Completed(Result)
}

/// Executes an async task until timeout of provided duration.
/// Returns an ExecuteTaskResult::Completed(Result) If the task was executed until completion where Result is the return type of the task
/// Returns an ExecuteTaskResult::Timeout otherwise.
pub async fn execute_until_timeout<Task, Duration>(task: Task, duration: Duration) -> TaskTimeoutResult<Task::Output>
    where Task: Future,
          Task::Output: Send,
          Duration: Into<std::time::Duration>,
{
    let timeout = tokio::time::sleep(duration.into());
    tokio::select! {
        _ = timeout => {
            TaskTimeoutResult::Timeout
        },
        r = task => {
            TaskTimeoutResult::Completed(r)
        }
    }
}

/// Executes an async task until timeout of provided duration on a different task.
/// Returns an ExecuteTaskResult::Completed(Result) If the task was executed until completion where Result is the return type of the task
/// Returns an ExecuteTaskResult::Timeout otherwise.
pub async fn execute_task_until_timeout<Task, Duration>(task: Task, duration: Duration) -> JoinHandle<TaskTimeoutResult<Task::Output>>
    where Task: Future + Send + 'static,
          Task::Output: Send,
          Duration: Into<std::time::Duration>,
{
    let timeout = tokio::time::sleep(duration.into());
    let f = async {
        tokio::select! {
        _ = timeout => {
            TaskTimeoutResult::Timeout
        },
        r = task => {
            TaskTimeoutResult::Completed(r)
        }
    }
    };
    tokio::task::spawn(f)
}