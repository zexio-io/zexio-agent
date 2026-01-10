use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::Stream;
use std::convert::Infallible;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use std::process::Stdio;
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;
use crate::{state::AppState, errors::AppError};

// Helper to stream logs from journalctl
// Uses kill_on_drop(true) to ensure the process is killed when the client disconnects.
pub async fn stream_journal_logs(unit_name: String) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    
    // Create a stream using async_stream or similar logic.
    // Since we need to yield from a spawned process, simpler to use a channel or `stream!` macro.
    // But `stream!` requires the `async-stream` crate which I haven't added.
    // I can use `futures::stream::unfold` or `try_stream` if avail.
    // Or I can just map a `LinesStream`.
    
    let mut cmd = Command::new("journalctl");
    cmd.arg("-u")
       .arg(&unit_name)
       .arg("-f") // Follow
       .arg("-n")
       .arg("100") // Start with last 100 lines
       .arg("--output=cat") // Cleaner output
       .arg("--no-pager")
       .stdout(Stdio::piped())
       .stderr(Stdio::piped()) // Capture stderr too? strictly stdout usually enough for logs
       .kill_on_drop(true); // CRITICAL: Kills process when handle is dropped

    let stream = async_stream::stream! {
        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(_) => return, // End stream if spawn fails
        };

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        let mut lines = LinesStream::new(reader.lines());

        while let Some(line) = lines.next().await {
            match line {
                Ok(l) => yield Ok(Event::default().data(l)),
                Err(_) => break,
            }
        }
        // When loop ends (process exits or stream dropped), child is dropped.
        // kill_on_drop ensures cleanup.
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}

// Handler for Project Logs
pub async fn project_logs_stream(
    State(_): State<AppState>,
    Path(project_id): Path<String>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let unit_name = format!("app@{}.service", project_id);
    stream_journal_logs(unit_name).await
}

// Handler for Worker Logs
pub async fn worker_logs_stream(
    State(_): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    stream_journal_logs("worker.service".to_string()).await
}
