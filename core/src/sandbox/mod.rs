use crate::Result;
use std::time::{Duration, Instant};

pub struct Sandbox {
    config: crate::SandboxConfig,
}

impl Sandbox {
    pub fn new(config: &crate::SandboxConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub fn execute(&self, command: &str, workdir: Option<&str>) -> Result<String> {
        let timeout = Duration::from_secs(30);
        let start = Instant::now();

        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = std::process::Command::new("cmd");
            c.args(["/C", command]);
            c
        } else {
            let mut c = std::process::Command::new("sh");
            c.args(["-c", command]);
            c
        };

        if let Some(dir) = workdir {
            cmd.current_dir(dir);
        }

        cmd.env("NEXUS_SANDBOX", "1");
        cmd.env("NEXUS_MAX_MEMORY_MB", format!("{}", self.config.max_memory_mb));

        if !self.config.enabled {
            let output = cmd.output().map_err(|e| crate::NexusError::Sandbox(e.to_string()))?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Ok(format!(
                "STDOUT:\n{}\nSTDERR:\n{}\nEXIT_CODE: {}",
                stdout, stderr,
                output.status.code().unwrap_or(-1)
            ));
        }

        let mut child = cmd.spawn().map_err(|e| crate::NexusError::Sandbox(e.to_string()))?;
        let pid = child.id();

        loop {
            if start.elapsed() > timeout {
                let _ = std::process::Command::new(if cfg!(target_os = "windows") { "taskkill" } else { "kill" })
                    .args(&[format!("/F /PID {}", pid)])
                    .status();
                let _ = child.kill();
                return Err(crate::NexusError::Sandbox(
                    "Command execution timed out (30s max)".to_string(),
                ));
            }

            match child.try_wait() {
                Ok(Some(status)) => {
                    let output = child.wait_with_output().map_err(|e| {
                        crate::NexusError::Sandbox(e.to_string())
                    })?;
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let truncate = |s: &str| -> String {
                        if s.len() > 100_000 {
                            format!("{}... [truncated from {} bytes]", &s[..100_000], s.len())
                        } else {
                            s.to_string()
                        }
                    };
                    return Ok(format!(
                        "STDOUT:\n{}\nSTDERR:\n{}\nEXIT_CODE: {}",
                        truncate(&stdout),
                        truncate(&stderr),
                        status.code().unwrap_or(-1)
                    ));
                }
                Ok(None) => {
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(e) => {
                    return Err(crate::NexusError::Sandbox(e.to_string()));
                }
            }
        }
    }
}
