mod tasks {
    use rox::models::{Task, Validate};
    fn build_default_task() -> Task {
        Task {
            name: String::from("test_task"),
            command: Some(String::from("some command")),
            uses: None,
            description: Some(String::from("This is a test task")),
            workdir: Some(String::from("rox/")),
            file_path: Some(String::from("some_filepath.yml")),
            values: None,
            hide: Some(false),
        }
    }

    #[test]
    fn valid_task_ok() {
        let task = build_default_task();
        assert!(task.validate().is_ok());
    }

    #[test]
    fn task_no_command_no_uses() {
        let mut task = build_default_task();
        task.command = None;
        task.uses = None;

        assert!(task.uses.is_none());
        assert!(task.command.is_none());
        assert!(task.values.is_none());

        let result = task.validate();
        assert!(
            result.is_err_and(|e| e.message == "A Task must implement either 'command' or 'uses'!")
        );
    }

    #[test]
    fn task_has_command_and_uses() {
        let mut task = build_default_task();
        task.uses = task.command.clone();

        // Confirm test setup
        assert!(task.uses.is_some());
        assert!(task.command.is_some());
        assert!(task.values.is_none());

        let result = task.validate();
        assert!(
            result.is_err_and(|e| e.message == "A Task cannot implement both 'command' & 'uses'!")
        );
    }

    #[test]
    fn task_has_uses_no_values() {
        let mut task = build_default_task();
        task.uses = task.command;
        task.command = None;

        // Confirm test setup
        assert!(task.uses.is_some());
        assert!(task.command.is_none());
        assert!(task.values.is_none());

        let result = task.validate();
        assert!(result.is_err_and(
            |e| e.message == "A Task that implements 'uses' must also implement 'values'!"
        ));
    }

    #[test]
    fn task_has_values_no_uses() {
        let mut task = build_default_task();
        task.values = Some(vec!["test".to_owned()]);

        // Confirm test setup
        assert!(task.uses.is_none());
        assert!(task.values.is_some());

        let result = task.validate();
        assert!(result.is_err_and(
            |e| e.message == "A Task that implements 'values' must also implement 'uses'!"
        ));
    }
}

mod templates {
    use rox::models::{Template, Validate};

    fn build_default_template() -> Template {
        Template {
            name: String::from("test_task"),
            command: String::from("docker build {path}"),
            symbols: vec!["{path}".to_owned()],
        }
    }

    #[test]
    fn valid_template_ok() {
        let template = build_default_template();
        assert!(template.validate().is_ok());
    }

    #[test]
    fn template_symbols_not_in_command() {
        let mut template = build_default_template();
        template.command = "some string".to_owned();

        let result = template.validate();
        assert!(result.is_err_and(
            |e| e.message == "A Template's 'symbols' must all exist within its 'command'!"
        ));
    }
}

mod version_requirements {
    use rox::models::{Validate, VersionRequirement};

    fn build_default_version_requirements() -> VersionRequirement {
        VersionRequirement {
            command: "python --version".to_string(),
            minimum_version: Some("3.8.0".to_string()),
            maximum_version: Some("3.13.0".to_string()),
            split: Some(true),
        }
    }

    #[test]
    fn valid_version_requirement_ok() {
        let version_requirement = build_default_version_requirements();
        assert!(version_requirement.validate().is_ok());
    }

    #[test]
    fn versions_not_valid() {
        let mut version_requirement = build_default_version_requirements();

        // Test Min Version
        let invalid_version = Some("1.a.0".to_owned());
        version_requirement.minimum_version = invalid_version.clone();
        version_requirement.maximum_version = None;

        let result = version_requirement.validate();
        assert!(result.is_err_and(
            |e| e.message == "Mininum and Maximum versions must be valid semantic version!"
        ));

        // Test Max Version
        let invalid_version = Some("1.a.0".to_owned());
        version_requirement.maximum_version = invalid_version.clone();
        version_requirement.minimum_version = None;

        let result = version_requirement.validate();
        assert!(result.is_err_and(
            |e| e.message == "Mininum and Maximum versions must be valid semantic version!"
        ));
    }

    #[test]
    fn min_ver_smaller_than_max_ver() {
        let mut version_requirement = build_default_version_requirements();

        version_requirement.minimum_version = Some("2.0.0".to_owned());
        version_requirement.maximum_version = Some("1.0.0".to_owned());

        let result = version_requirement.validate();
        assert!(result.is_err_and(
            |e| e.message == "The Minimum version cannot be larger than the Maximum version!"
        ));
    }

    #[test]
    fn if_split_must_include_a_version_constraint() {
        let mut version_requirement = build_default_version_requirements();

        version_requirement.split = Some(true);
        version_requirement.minimum_version = None;
        version_requirement.maximum_version = None;

        let result = version_requirement.validate();
        assert!(result.is_err_and(
            |e| e.message == "If 'split' is defined, either a 'minimum_version' or a 'maximum_version' is also required!"
        ));
    }
}
