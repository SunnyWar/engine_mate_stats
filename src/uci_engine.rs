use anyhow::{Result, anyhow};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct UciEngine {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl UciEngine {
    /// Start a new UCI engine process from the given executable path.
    pub fn start(engine_path: &str) -> Result<Self> {
        let mut child = Command::new(engine_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to start engine: {}", e))?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow!("Failed to open stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("Failed to open stdout"))?;
        let stdout = BufReader::new(stdout);

        Ok(UciEngine {
            child,
            stdin,
            stdout,
        })
    }

    /// Send a raw UCI command string (for compatibility).
    pub fn send_command(&mut self, command: &str) -> Result<()> {
        writeln!(self.stdin, "{}", command)?;
        self.stdin.flush()?;
        Ok(())
    }

    /// Read a line of output from the engine as a raw string.
    pub fn read_line(&mut self) -> Result<String> {
        let mut line = String::new();
        let n = self.stdout.read_line(&mut line)?;
        if n == 0 {
            return Err(anyhow!("Engine process closed output"));
        }
        Ok(line.trim_end().to_string())
    }
}

impl Drop for UciEngine {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
