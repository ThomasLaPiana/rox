use crate::syntax::VersionRequirements;
use std::process::{Command, Output};

fn split_head_from_rest(snek: String) -> (String, Vec<String>) {
    let mut split_snek = snek.split(" ");
    let head: String = split_snek.next().unwrap().to_owned();
    let remainder: Vec<String> = split_snek.map(|x| x.to_owned()).collect();
    (head, remainder)
}

pub fn check_version(requirements: VersionRequirements) {
    let (command, args) = split_head_from_rest(requirements.command);
    let command_output: Output = Command::new(command)
        .args(args)
        .output()
        .expect("Command Failed!");
    let version = std::str::from_utf8(&command_output.stdout)
        .unwrap()
        .strip_suffix("\n")
        .unwrap();
    println!("{:?}", version);
}
