use std::sync::{Arc, Weak};
use parking_lot::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Pending,
    Loading,
    Error,
    Warning,
    Success,
}

#[derive(Debug, Clone)]
pub struct TaskData {
    pub title: String,
    pub state: TaskState,
    pub status: Option<String>,
    pub output: Option<String>,
    pub children: Vec<Arc<Task>>,
}

#[derive(Debug)]
pub struct Task {
    pub data: Mutex<TaskData>,
    pub parent: Option<Weak<Task>>,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            data: Mutex::new(TaskData {
                title,
                state: TaskState::Pending,
                status: None,
                output: None,
                children: Vec::new(),
            }),
            parent: None,
        }
    }

    pub fn add_child(parent: &Arc<Task>, title: String) -> Arc<Task> {
        let child = Arc::new(Task {
            data: Mutex::new(TaskData {
                title,
                state: TaskState::Pending,
                status: None,
                output: None,
                children: Vec::new(),
            }),
            parent: Some(Arc::downgrade(parent)),
        });
        
        parent.data.lock().children.push(child.clone());
        child
    }
}

pub struct TaskInnerApi {
    pub(crate) task: Arc<Task>,
    pub(crate) renderer_signal: Arc<tokio::sync::Notify>,
}

impl TaskInnerApi {
    pub fn set_title(&self, title: impl Into<String>) {
        self.task.data.lock().title = title.into();
        self.renderer_signal.notify_waiters();
    }

    pub fn set_status(&self, status: impl Into<String>) {
        self.task.data.lock().status = Some(status.into());
        self.renderer_signal.notify_waiters();
    }

    pub fn set_output(&self, output: impl Into<String>) {
        self.task.data.lock().output = Some(output.into());
        self.renderer_signal.notify_waiters();
    }

    pub fn set_warning(&self, warning: impl Into<String>) {
        let mut data = self.task.data.lock();
        data.state = TaskState::Warning;
        data.output = Some(warning.into());
        self.renderer_signal.notify_waiters();
    }

    pub fn set_error(&self, error: impl Into<String>) {
        let mut data = self.task.data.lock();
        data.state = TaskState::Error;
        data.output = Some(error.into());
        self.renderer_signal.notify_waiters();
    }
}
