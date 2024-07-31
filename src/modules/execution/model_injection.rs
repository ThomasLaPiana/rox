use crate::models;

/// Get used Template's information and inject set values
pub fn inject_template_values(mut task: models::Task, template: &models::Template) -> models::Task {
    task.command = {
        let mut template_command = template.command.to_owned();
        let template_symbols = template.symbols.to_owned();
        let task_values = task.values.as_ref().unwrap();

        for i in 0..task_values.len() {
            template_command = template_command.replace(
                template_symbols.get(i).unwrap(),
                task_values.get(i).unwrap(),
            );
        }
        Some(template_command)
    };
    task
}

#[test]
fn inject_template_values_valid() {
    let test_task = models::Task {
        name: "Test".to_string(),
        command: None,
        file_path: None,
        uses: None,
        values: Some(vec!["1".to_owned(), "2".to_owned()]),
        description: None,
        hide: None,
        workdir: None,
    };
    let test_template = models::Template {
        name: "Test".to_string(),
        command: "This is {one} and {two}".to_owned(),
        symbols: vec!["{one}".to_owned(), "{two}".to_owned()],
    };
    let output_task = inject_template_values(test_task, &test_template);
    assert_eq!(output_task.command.unwrap(), "This is 1 and 2".to_owned())
}

/// Inject additional metadata into each Task and sort based on name.
pub fn inject_task_metadata(tasks: Vec<models::Task>, file_path: &str) -> Vec<models::Task> {
    let mut sorted_tasks: Vec<models::Task> = tasks
        .into_iter()
        .map(|mut task| {
            task.file_path = Some(file_path.to_owned());

            if task.description.is_none() {
                task.command.clone_into(&mut task.description)
            }
            task
        })
        .collect();
    sorted_tasks.sort_by(|x, y| x.name.to_lowercase().cmp(&y.name.to_lowercase()));
    sorted_tasks
}
