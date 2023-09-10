use crate::syntax::VersionRequirement;
use crate::utils;
use semver::{Version, VersionReq};
use std::{
    process::{Command, Output},
    str::FromStr,
};

// Run the command and get the version output
fn get_version_output(command: String, args: Vec<String>, split_output: bool) -> String {
    let command_output: Output = Command::new(command)
        .args(args)
        .output()
        .expect("Command Failed!");

    let mut version = std::str::from_utf8(&command_output.stdout).unwrap().trim();

    if split_output {
        version = version.split(' ').last().unwrap();
    }
    version.to_owned()
}

#[derive(PartialEq, Debug)]
enum VersionCheck {
    BelowMin,
    AboveMax,
    Valid,
}

// Parse Strings into Versions and Compare
fn compare_versions(
    version: String,
    minver: Option<String>,
    maxver: Option<String>,
) -> VersionCheck {
    let parsed_version = Version::from_str(&version).expect("Failed to parse version string!");

    // Check Min Version Constraints
    if let Some(unwrapped_minver) = minver {
        println!("> Minimum Version Required: {}", unwrapped_minver);
        let parsed_minver = VersionReq::from_str(&format!("> {}", &unwrapped_minver))
            .expect("Failed to parse minimum version!");
        let result = parsed_minver.matches(&parsed_version);
        if !result {
            return VersionCheck::BelowMin;
        }
    }

    // Check Max Version Constraints
    if let Some(unwrapped_maxver) = maxver {
        println!("> Maximum Version Expected: {}", unwrapped_maxver);
        let parsed_minver = VersionReq::from_str(&format!("< {}", &unwrapped_maxver))
            .expect("Failed to parse maximum version!");
        let result = parsed_minver.matches(&parsed_version);
        if !result {
            return VersionCheck::AboveMax;
        }
    }

    VersionCheck::Valid
}

pub fn check_version(requirements: VersionRequirement) {
    println!(
        "> Checking version for command:\n\t{}",
        &requirements.command
    );
    let (command, args) = utils::split_head_from_rest(requirements.command);
    let version = get_version_output(command, args, requirements.split.is_some());
    println!("> Found version: {}", version);

    let minver = requirements.minimum_version;
    let maxver = requirements.maximum_version;

    let version_check_result = compare_versions(version, minver, maxver);

    match version_check_result {
        VersionCheck::Valid => utils::color_print(
            vec!["Version Check succeeded!".to_string()],
            utils::ColorEnum::Green,
        ),
        VersionCheck::AboveMax => utils::color_print(
            vec!["Exceeded maximum version!".to_string()],
            utils::ColorEnum::Red,
        ),
        VersionCheck::BelowMin => utils::color_print(
            vec!["Below minimum version!".to_string()],
            utils::ColorEnum::Red,
        ),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_version() {
        let result = compare_versions(
            "2.2.7".to_string(),
            Some("1.2.4".to_string()),
            Some("2.2.8".to_string()),
        );
        assert_eq!(result, VersionCheck::Valid)
    }

    #[test]
    fn under_min_version() {
        let result = compare_versions("1.2.3".to_string(), Some("1.2.4".to_string()), None);
        assert_eq!(result, VersionCheck::BelowMin)
    }

    #[test]
    fn over_max_version() {
        let result = compare_versions(
            "2.2.7".to_string(),
            Some("1.2.4".to_string()),
            Some("1.2.8".to_string()),
        );
        assert_eq!(result, VersionCheck::AboveMax)
    }
}
