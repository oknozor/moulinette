use shiplift::{tty::StreamType, ExecContainerOptions};
use shiplift::{Docker, PullOptions};
use tokio::prelude::{Future, Stream};

pub(crate) fn pull_image() {
    env_logger::init();

    let docker = Docker::new();
    let img = "moulinette/rust";
    let pull_option = PullOptions::builder().image(img).tag("latest").build();

    let fut = docker
        .images()
        .pull(&pull_option)
        .for_each(|output| {
            println!("{:?}", output);
            Ok(())
        })
        .map_err(|e| eprintln!("Error: {}", e));
    tokio::run(fut);
}

pub(crate) fn run_container() {
    let docker = Docker::new();

    let id = "rust_stable_moulinette";

    let options = ExecContainerOptions::builder()
        .cmd(vec![
            "sh",
            "-c",
            "cargo fmt --all",
            "cargo build",
        ])
        .env(vec!["VAR=value"])
        .attach_stdout(true)
        .attach_stderr(true)
        .build();

    let container = docker.containers().get(&id);

    let fut = container
        .exec(&options)
        .for_each(|chunk| {
            match chunk.stream_type {
                StreamType::StdOut => println!("Stdout: {}", chunk.as_string_lossy()),
                StreamType::StdErr => eprintln!("Stderr: {}", chunk.as_string_lossy()),
                StreamType::StdIn => unreachable!(),
            }
            Ok(())
        })
        .map_err(|e| eprintln!("Error: {}", e));

    tokio::run(fut);
}
