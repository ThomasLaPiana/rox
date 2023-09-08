use crate::syntax::VersionRequirements;
use crate::utils;
use semver::{Version, VersionReq};
use std::{
    process::{Command, Output},
    str::FromStr,
};

pub fn check_version(requirements: VersionRequirements) {
    println!(
        "> Checking version for command:\n\t{}",
        &requirements.command
    );
    let (command, args) = utils::split_head_from_rest(requirements.command);
    let command_output: Output = Command::new(command)
        .args(args)
        .output()
        .expect("Command Failed!");

    let mut version = std::str::from_utf8(&command_output.stdout).unwrap().trim();

    if requirements.split.is_some_and(|x| x) {
        version = version.split(' ').last().unwrap();
    }
    println!("> Found version: {}", version);

    let parsed_version = Version::from_str(version).expect("Failed to parse version string!");
    let minver = requirements.minimum_version;
    let maxver = requirements.maximum_version;

    if let Some(unwrapped_minver) = minver {
        println!("> Minimum Version Required: {}", unwrapped_minver);
        let parsed_minver = VersionReq::from_str(&format!("> {}", &unwrapped_minver))
            .expect("Failed to parse minimum version!");
        let result = parsed_minver.matches(&parsed_version);
        if !result {
            // TODO: Convert this panic to a RED print
            panic!("Minimum Version Mismatch!");
        }
    }

    if let Some(unwrapped_maxver) = maxver {
        println!("> Maximum Version Expected: {}", unwrapped_maxver);
        let parsed_minver = VersionReq::from_str(&format!("< {}", &unwrapped_maxver))
            .expect("Failed to parse maximum version!");
        let result = parsed_minver.matches(&parsed_version);
        if !result {
            // TODO: Convert this panic to a RED print
            panic!("Maximum Version Mismatch!");
        }
    }
    utils::color_print(
        vec!["Version Check succeeded!".to_string()],
        utils::ColorEnum::Green,
    );
}

#[test]
fn valid_versions() {
    todo!()
}

#[test]
fn under_min_version() {
    todo!()
}

#[test]
fn over_max_version() {
    todo!()
}
