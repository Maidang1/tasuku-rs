# tasuku-rs

轻量级任务管理与终端渲染库，受 tasuku 启发，使用 Rust 与 Tokio 构建。支持任务状态更新、层级任务、在支持 TTY 的终端下进行渐进式渲染，并在 CI 环境中以纯文本输出。

## 特性
- 任务状态：`Pending`、`Loading`、`Success`、`Warning`、`Error`
- 终端渲染：自动检测颜色支持与 CI 环境
- 层级结构：任务可包含子任务与状态输出
- 易用 API：通过异步函数定义任务流程

## 安装与构建
- 依赖：`tokio`、`crossterm`、`atty`
- 使用 `cargo build` 构建，使用 `cargo run` 运行示例代码

## 快速上手
```rust
use tasuku::{Tasuku, TaskState};

#[tokio::main]
async fn main() {
    let t = Tasuku::new();

    let task = t.task("示例任务", |api| async move {
        api.set_status(Some("处理中".to_string()));
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        api.set_output("输出内容");
    }).await;

    assert_eq!(task.state(), TaskState::Success);
}
```

## 许可证
本项目采用 MIT 许可证，详见 `LICENSE` 文件。
