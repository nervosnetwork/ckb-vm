use std::process::{self, Command};

pub fn get_current_memory() -> usize {
    let pid = format!("{}", process::id());
    let output = String::from_utf8(
        Command::new("ps")
            .arg("-p")
            .arg(pid)
            .arg("-o")
            .arg("rss")
            .output()
            .expect("run ps failed")
            .stdout,
    )
    .unwrap();

    let output = output.split("\n").collect::<Vec<&str>>();

    let memory_size = output[1].replace(" ", "");
    memory_size.parse().unwrap()
}
