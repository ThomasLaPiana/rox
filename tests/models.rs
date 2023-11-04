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
    assert!(result.is_err());
    assert!(result.is_err_and(|e| e.message == "A Task must implement either 'command' or 'uses'!"));
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
    assert!(result.is_err());
    assert!(result.is_err_and(|e| e.message == "A Task cannot implement both 'command' & 'uses'!"));
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
    assert!(result.is_err());
    assert!(
        result.is_err_and(
            |e| e.message == "A Task that implements 'uses' must also implement 'values'!"
        )
    );
}

#[test]
fn task_has_values_no_uses() {
    let mut task = build_default_task();
    task.values = Some(vec!["test".to_owned()]);

    // Confirm test setup
    assert!(task.uses.is_none());
    assert!(task.values.is_some());

    let result = task.validate();
    assert!(result.is_err());
    assert!(
        result.is_err_and(
            |e| e.message == "A Task that implements 'values' must also implement 'uses'!"
        )
    );
}
