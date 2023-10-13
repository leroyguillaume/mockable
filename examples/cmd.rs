use mockable::{Command, CommandRunner, DefaultCommandRunner};

async fn echo(msg: &str, runner: &dyn CommandRunner) -> String {
    let cmd = Command::new("echo".into()).with_arg(msg.into());
    let output = runner.run(cmd).await.expect("echo failed");
    String::from_utf8(output.stdout).expect("echo output is not utf8")
}

#[tokio::main]
async fn main() {
    let msg = echo("Hello, world!", &DefaultCommandRunner).await;
    println!("{msg}");
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
        let message = echo(&expected, &runner).await;
        assert_eq!(message, expected);
    }
}
