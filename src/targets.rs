use crate::syntax::Target;
use crate::utils;
use std::{
    process::{Command, Output},
};

pub fn run_target(target: &Target) {
    let (command, args) = utils::split_head_from_rest(target.command.clone());
    let command_output: Output = Command::new(command)
        .args(args)
        .output()
        .expect("Command Failed!");
    // TODO: Handle stderr
    let output = std::str::from_utf8(&command_output.stdout).unwrap().trim();
    println!("{}", output);
}
