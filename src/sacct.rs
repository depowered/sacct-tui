use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Deserialize)]
pub struct SacctData {
    #[serde(rename = "JobID")]
    pub job_id: String,
    #[serde(rename = "JobName")]
    pub job_name: String,
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "Start")]
    pub start: String,
    #[serde(rename = "End")]
    pub end: String,
    #[serde(rename = "Elapsed")]
    pub elapsed: String,
    #[serde(rename = "Timelimit")]
    pub time_limit: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "NNodes")]
    pub n_nodes: String,
    #[serde(rename = "ExitCode")]
    pub exit_code: String,
    #[serde(rename = "NodeList")]
    pub node_list: String,
    #[serde(rename = "ReqCPUS")]
    pub req_cpus: String,
    #[serde(rename = "ReqMem")]
    pub req_mem: String,
    #[serde(rename = "MaxRSS")]
    pub max_rss: String,
    #[serde(rename = "UserCPU")]
    pub user_cpu: String,
}

impl SacctData {
    pub fn display_line(&self) -> String {
        format!(
            "{:<12} {:<20} {:<10} {:<12} {:<8}",
            self.job_id, self.job_name, self.state, self.elapsed, self.user
        )
    }
}

pub fn fetch_sacct_data(
    additional_args: Option<String>,
) -> Result<Vec<SacctData>, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("sacct");

    cmd.args([
        "--format=JobID,JobName,User,Start,End,Elapsed,Timelimit,State,NNodes,ExitCode,NodeList,ReqCPUS,ReqMem,MaxRSS,UserCPU",
        "--user=$USER",
        "--allocations",
        "--parsable2",
        "--delimiter=,",
        "--noheader",
    ]);

    if let Some(args) = additional_args {
        let arg_parts: Vec<&str> = args.split_whitespace().collect();
        cmd.args(arg_parts);
    }

    let output = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("sacct command failed: {}", stderr).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_csv_data(&stdout)
}

pub fn read_csv_file(file_path: &str) -> Result<Vec<SacctData>, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(file_path)?;
    parse_csv_data(&file_content)
}

fn parse_csv_data(csv_content: &str) -> Result<Vec<SacctData>, Box<dyn std::error::Error>> {
    // Check if first line looks like headers
    let has_headers = csv_content
        .lines()
        .next()
        .map(is_header_line)
        .unwrap_or(false);

    let mut reader = ReaderBuilder::new()
        .has_headers(has_headers)
        .from_reader(csv_content.as_bytes());

    let mut jobs = Vec::new();
    for result in reader.deserialize() {
        let job: SacctData = result?;
        jobs.push(job);
    }

    Ok(jobs)
}

fn is_header_line(line: &str) -> bool {
    let lower_line = line.to_lowercase();
    // Check if line contains expected header field names
    lower_line.contains("jobid") || lower_line.contains("jobname") || lower_line.contains("state")
}
