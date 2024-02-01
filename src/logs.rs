use crate::models::JobResults;
use crate::output::display_execution_results;

const LOG_DIR: &str = ".rox";

/// Load execution results from a log file
pub fn display_logs(number: &i8) {
    let mut filenames = std::fs::read_dir(LOG_DIR)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    filenames.sort();

    let results: Vec<JobResults> = filenames
        .iter()
        .rev()
        .take(*number as usize)
        .map(|filename| {
            let contents = std::fs::read_to_string(filename).unwrap();
            serde_yaml::from_str(&contents).unwrap()
        })
        .collect();

    for result in results.iter().rev() {
        println!("\n> {} | {}", result.job_name, result.execution_time);
        display_execution_results(result)
    }
}

/// Write the execution results to a log file
pub fn write_logs(results: &JobResults) -> String {
    let filename = format!("rox-{}.log.yaml", chrono::Utc::now().to_rfc3339());
    let filepath = format!("{}/{}", LOG_DIR, filename);

    // Make sure the log directory exists
    if !std::path::Path::new(LOG_DIR).exists() {
        std::fs::create_dir(LOG_DIR).unwrap();
    }

    std::fs::write(filepath, serde_yaml::to_string(results).unwrap()).unwrap();
    filename
}
