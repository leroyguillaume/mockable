use std::{collections::HashMap, io::Result, path::PathBuf, process::Output};

use async_trait::async_trait;
use tracing::trace;

// Command

/// A command.
///
/// **This is supported on `feature=cmd` only.**
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Command {
    /// The arguments to pass to the command.
    pub args: Vec<String>,
    /// The current working directory to run the command in.
    pub cwd: Option<PathBuf>,
    /// The environment variables to set for the command.
    pub env: Option<HashMap<String, String>>,
    /// The group to run the command as.
    #[cfg(unix)]
    pub gid: Option<u32>,
    /// The program to run.
    pub program: String,
    /// The user to run the command as.
    #[cfg(unix)]
    pub uid: Option<u32>,
}

// CommandOutput

/// The output of a command.
///
/// **This is supported on `feature=cmd` only.**
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommandOutput {
    /// The exit code of the command.
    pub code: Option<i32>,
    /// The standard error output of the command.
    pub stderr: Vec<u8>,
    /// The standard output of the command.
    pub stdout: Vec<u8>,
}

impl From<Output> for CommandOutput {
    fn from(output: Output) -> Self {
        Self {
            code: output.status.code(),
            stderr: output.stderr,
            stdout: output.stdout,
        }
    }
}

// CommandRunner

/// A trait for running commands.
///
/// **This is supported on `feature=cmd` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/cmd.rs).
#[async_trait]
pub trait CommandRunner: Send + Sync {
    /// Runs the given command.
    async fn run(&self, cmd: Command) -> Result<CommandOutput>;
}

// DefaultCommandRunner

/// Default implementation of [`CommandRunner`](trait.CommandRunner.html).
///
/// **This is supported on `feature=cmd` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/cmd.rs).
pub struct DefaultCommandRunner;

#[async_trait]
impl CommandRunner for DefaultCommandRunner {
    async fn run(&self, cmd: Command) -> Result<CommandOutput> {
        trace!(?cmd, "running command");
        let mut builder = tokio::process::Command::new(cmd.program);
        builder.args(cmd.args);
        if let Some(cwd) = cmd.cwd {
            builder.current_dir(cwd);
        }
        if let Some(env) = cmd.env {
            builder.envs(env);
        }
        if cfg!(unix) {
            if let Some(gid) = cmd.gid {
                builder.gid(gid);
            }
            if let Some(uid) = cmd.uid {
                builder.uid(uid);
            }
        }
        let output = builder.output().await?;
        Ok(output.into())
    }
}

// MockCommandRunner

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of [`CommandRunner`](trait.CommandRunner.html).
    ///
    /// **This is supported on `feature=cmd,mock` only.**
    ///
    /// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/cmd.rs).
    pub CommandRunner {}

    #[async_trait]
    impl CommandRunner for CommandRunner {
        async fn run(&self, cmd: Command) -> Result<CommandOutput>;
    }
}
