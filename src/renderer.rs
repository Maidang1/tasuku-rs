use std::io::{self, Write};
use std::sync::Arc;
use crossterm::{
    cursor,
    style::Stylize,
    style,
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use crate::types::{Task, TaskState};

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct Renderer {
    pub tasks: Vec<Arc<Task>>,
    spinner_frame: usize,
    last_line_count: u16,
}

impl Renderer {
    pub fn new(tasks: Vec<Arc<Task>>) -> Self {
        Self {
            tasks,
            spinner_frame: 0,
            last_line_count: 0,
        }
    }

    pub fn update_tasks(&mut self, tasks: Vec<Arc<Task>>) {
        self.tasks = tasks;
    }

    pub fn render(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Clear previous output
        if self.last_line_count > 0 {
            stdout.queue(cursor::MoveUp(self.last_line_count))?;
            stdout.queue(Clear(ClearType::FromCursorDown))?;
        }

        let mut lines = Vec::new();
        for task in &self.tasks {
            self.render_task(task, 0, &mut lines);
        }

        for line in &lines {
            stdout.queue(style::Print(line))?;
            stdout.queue(style::Print("\n"))?;
        }

        stdout.flush()?;
        self.last_line_count = lines.len() as u16;
        self.spinner_frame = (self.spinner_frame + 1) % SPINNER_FRAMES.len();

        Ok(())
    }

    fn render_task(&self, task: &Arc<Task>, depth: usize, lines: &mut Vec<String>) {
        let data = task.data.lock();
        let indent = "  ".repeat(depth);
        
        let icon = match data.state {
            TaskState::Pending => "◼".grey(),
            TaskState::Loading => {
                if !data.children.is_empty() {
                    "❯".yellow()
                } else {
                    SPINNER_FRAMES[self.spinner_frame].yellow()
                }
            }
            TaskState::Success => {
                if !data.children.is_empty() {
                    "❯".yellow()
                } else {
                    "✔".green()
                }
            }
            TaskState::Error => {
                if !data.children.is_empty() {
                    "❯".red()
                } else {
                    "✖".red()
                }
            }
            TaskState::Warning => "⚠".yellow(),
        };

        let mut line = format!("{}{}", indent, icon);
        line.push_str(&format!(" {}", data.title));

        if let Some(status) = &data.status {
            line.push_str(&format!(" {}", format!("[{}]", status).dim()));
        }

        lines.push(line);

        if let Some(output) = &data.output {
            let output_indent = format!("{}  ", indent);
            for (i, output_line) in output.lines().enumerate() {
                let prefix = if i == 0 { "→ " } else { "" };
                lines.push(format!("{}{}{}", output_indent, prefix, output_line.grey()));
            }
        }

        for child in &data.children {
            self.render_task(child, depth + 1, lines);
        }
    }
}
