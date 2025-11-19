pub mod types;
pub mod renderer;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::future::Future;
use parking_lot::Mutex;
use tokio::sync::Notify;
use once_cell::sync::Lazy;

use crate::types::{Task, TaskInnerApi, TaskState};
use crate::renderer::Renderer;

// Global task list for the root level
static ROOT_TASKS: Lazy<Mutex<Vec<Arc<Task>>>> = Lazy::new(|| Mutex::new(Vec::new()));
static RENDERER_NOTIFY: Lazy<Arc<Notify>> = Lazy::new(|| Arc::new(Notify::new()));
static RENDERER_STARTED: AtomicBool = AtomicBool::new(false);

pub async fn task<F, Fut, T>(title: impl Into<String>, f: F) -> T
where
    F: FnOnce(TaskInnerApi) -> Fut,
    Fut: Future<Output = T>,
{
    let title = title.into();
    let task = Arc::new(Task::new(title));
    
    {
        let mut root = ROOT_TASKS.lock();
        root.push(task.clone());
    }

    // Start the renderer loop if it hasn't started yet
    if !RENDERER_STARTED.swap(true, Ordering::SeqCst) {
        let renderer_notify = RENDERER_NOTIFY.clone();
        tokio::spawn(async move {
            let mut renderer = Renderer::new(ROOT_TASKS.lock().clone());
            
            // Hide cursor
            let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide);

            loop {
                // Wait for notification or timeout for spinner animation
                let _ = tokio::time::timeout(std::time::Duration::from_millis(80), renderer_notify.notified()).await;
                
                // Update renderer's task list reference
                {
                     let current_root = ROOT_TASKS.lock();
                     if current_root.len() != renderer.tasks.len() {
                         renderer.update_tasks(current_root.clone());
                     }
                }

                if let Err(_) = renderer.render() {
                    break;
                }
                
                // TODO: Add exit condition logic if needed, but for now it runs forever in this process
            }
            
            // Show cursor (this might not be reached if we don't break)
            let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Show);
        });
    }

    let api = TaskInnerApi {
        task: task.clone(),
        renderer_signal: RENDERER_NOTIFY.clone(),
    };

    {
        let mut data = task.data.lock();
        data.state = TaskState::Loading;
    }
    RENDERER_NOTIFY.notify_waiters();

    let result_wrapper = std::panic::AssertUnwindSafe(f(api));
    let result = match futures::FutureExt::catch_unwind(result_wrapper).await {
        Ok(res) => res,
        Err(_) => {
             let mut data = task.data.lock();
             data.state = TaskState::Error;
             data.output = Some("Task panicked".to_string());
             RENDERER_NOTIFY.notify_waiters();
             panic!("Task panicked");
        }
    };

    {
        let mut data = task.data.lock();
        if data.state == TaskState::Loading {
            data.state = TaskState::Success;
        }
    }
    RENDERER_NOTIFY.notify_waiters();

    result
}
