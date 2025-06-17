use std::io::{self, BufRead, BufReader, Write};
use tokio::sync::mpsc;
use serde_json::Value;

use crate::Result;

/// Stdio transport for MCP communication
pub struct StdioTransport {
    tx: mpsc::Sender<Value>,
    rx: mpsc::Receiver<Value>,
}

impl StdioTransport {
    pub fn new() -> (Self, mpsc::Receiver<Value>, mpsc::Sender<Value>) {
        let (stdin_tx, stdin_rx) = mpsc::channel(100);
        let (stdout_tx, stdout_rx) = mpsc::channel(100);
        
        let transport = Self {
            tx: stdout_tx.clone(),
            rx: stdin_rx,
        };
        
        (transport, stdout_rx, stdin_tx)
    }
    
    /// Start reading from stdin
    pub fn start_stdin_reader(tx: mpsc::Sender<Value>) {
        std::thread::spawn(move || {
            let stdin = io::stdin();
            let reader = BufReader::new(stdin);
            
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        if line.trim().is_empty() {
                            continue;
                        }
                        
                        match serde_json::from_str::<Value>(&line) {
                            Ok(value) => {
                                if tx.blocking_send(value).is_err() {
                                    break;
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to parse JSON: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to read line: {}", e);
                        break;
                    }
                }
            }
        });
    }
    
    /// Start writing to stdout
    pub fn start_stdout_writer(mut rx: mpsc::Receiver<Value>) {
        tokio::spawn(async move {
            let mut stdout = io::stdout();
            
            while let Some(value) = rx.recv().await {
                match serde_json::to_string(&value) {
                    Ok(json) => {
                        if let Err(e) = writeln!(stdout, "{}", json) {
                            tracing::error!("Failed to write to stdout: {}", e);
                            break;
                        }
                        if let Err(e) = stdout.flush() {
                            tracing::error!("Failed to flush stdout: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to serialize JSON: {}", e);
                    }
                }
            }
        });
    }
    
    pub async fn send(&self, message: Value) -> Result<()> {
        self.tx.send(message).await
            .map_err(|_| crate::McpImageError::Mcp("Failed to send message".to_string()))?;
        Ok(())
    }
    
    pub async fn recv(&mut self) -> Option<Value> {
        self.rx.recv().await
    }
}