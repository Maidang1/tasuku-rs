use std::time::Duration;
use tasuku_rs::task;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    task("Loading dependencies", |task| async move {
        task.set_status("installing...");
        sleep(Duration::from_millis(1500)).await;
        task.set_status("finished");
        task.set_output("Installed 5 packages");
    })
    .await;

    task("Running tests", |task| async move {
        sleep(Duration::from_millis(1000)).await;
        task.set_warning("Some tests are flaky");
    })
    .await;

    task("Deploying", |task| async move {
        task.set_status("uploading...");
        sleep(Duration::from_millis(1000)).await;

        // Nested task simulation (API not fully exposed yet for nesting in this v1, but let's try sequential)
        task.set_output("Upload complete");
        task.set_status("verifying...");
        sleep(Duration::from_millis(1000)).await;
    })
    .await;

    // Keep the process alive a bit to see the final state
    sleep(Duration::from_millis(500)).await;
}
