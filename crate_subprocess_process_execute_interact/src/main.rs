use std::error::Error;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use subprocess::{Exec, ExitStatus, Popen, PopenConfig, Redirection};

fn main() -> Result<(), Box<dyn Error>> {
    // Start a subprocess and obtain its output as a Read trait object, like C's popen:
    let stream = Exec::cmd("find").arg("src").stream_stdout()?;
    for i in BufReader::new(stream).lines() {
        println!("{}", i?)
    }

    // To execute a command using the OS shell, like C's system, use Exec::shell
    Exec::shell("echo are you okay?").join()?;

    // Capture the output of a command
    let out = Exec::cmd("ls")
        .stdout(Redirection::Pipe)
        .capture()?
        .stdout_str();
    println!("{}", out);

    // Redirect standard error to standard output, and capture them in a string
    let out_and_err = Exec::cmd("ls")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()?
        .stdout_str();
    println!("{}", out_and_err);

    // Provide some input to the command and read its output
    let out = Exec::cmd("sort")
        .stdin("b\nc\na\n")
        .stdout(Redirection::Pipe)
        .capture()?
        .stdout_str();
    assert_eq!(out, "a\nb\nc\n");

    // Execute a pipeline and return the exit status of the last command
    let exit_status = (Exec::shell("ls *.bak") | Exec::cmd("xargs").arg("rm")).join()?;
    assert_eq!(exit_status, ExitStatus::Exited(123));

    // Capture the pipeline's output
    let out = { Exec::shell("find . -type f") | Exec::cmd("sort") | Exec::cmd("sha1sum") }
        .capture()?
        .stdout_str();
    println!("{}", out);

    // The low-level Popen type
    let mut p = Popen::create(
        &["md5sum", "Cargo.toml"],
        PopenConfig {
            stdout: Redirection::Pipe,
            ..Default::default()
        },
    )?;
    // Since we requested stdout to be redirected to a pipe, the parent's
    // end of the pipe is available as p.stdout.  It can either be read
    // directly, or processed using the communicate() method:
    let (out, err) = p.communicate(None)?;
    println!("out message:\n{:?}\nerror message:\n{:?}", out, err);
    // check if the process is still alive
    if let Some(_exit_status) = p.poll() {
        // the process has finished
    } else {
        // it is still running, terminate it
        p.terminate()?;
    }

    // Check whether a previously launched process is still running
    let mut p = Exec::cmd("sleep").arg("2").popen()?;
    thread::sleep(Duration::new(1, 0));
    if p.poll().is_none() {
        // poll() returns Some(exit_status) if the process has completed
        println!("process is still running");
    }

    // Give the process 1 second to run, and kill it if it didn't complete by then
    let mut p = Exec::cmd("sleep").arg("2").popen()?;
    if let Some(status) = p.wait_timeout(Duration::new(1, 0))? {
        println!("process finished as {:?}", status);
    } else {
        p.kill()?;
        p.wait()?;
        println!("process killed");
    };
    Ok(())
}
