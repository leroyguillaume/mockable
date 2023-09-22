use mockable::{Command, CommandRunner, DefaultCommandRunner};

struct EchoRunner(Box<dyn CommandRunner>);

impl EchoRunner {
    fn new() -> Self {
        Self(Box::new(DefaultCommandRunner))
    }

    async fn echo(&self, message: &str) -> String {
        let cmd = Command::new("echo".into()).with_arg(message.into());
        let output = self.0.run(cmd).await.expect("echo failed");
        String::from_utf8(output.stdout).expect("echo output is not utf8")
    }
}

#[tokio::main]
async fn main() {
    let runner = EchoRunner::new();
    let message = runner.echo("Hello, world!").await;
    println!("{message}");
}

#[cfg(test)]
mod test {
    use mockable::{CommandOutput, MockCommandRunner};
    use mockall::predicate::eq;

    use super::*;

    #[tokio::test]
    async fn test() {
        let expected = "Hello, world!";
        let cmd = Command::new("echo".into()).with_arg(expected.into());
        let mut runner = MockCommandRunner::new();
        runner.expect_run().with(eq(cmd)).returning(|_| {
            Ok(CommandOutput {
                code: Some(0),
                stderr: vec![],
                stdout: expected.as_bytes().to_vec(),
            })
        });
        let runner = EchoRunner(Box::new(runner));
        let message = runner.echo(expected).await;
        assert_eq!(message, expected);
    }
}
