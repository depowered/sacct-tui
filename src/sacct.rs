use serde::Deserialize;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Deserialize)]
pub struct SacctData {
    pub job_id: String,
    pub job_name: String,
    pub partition: String,
    pub account: String,
    pub alloc_cpus: String,
    pub state: String,
    pub exit_code: String,
    pub start: String,
    pub end: String,
    pub elapsed: String,
    pub time_limit: String,
    pub submit: String,
    pub user: String,
    pub work_dir: String,
}

impl SacctData {
    pub fn from_line(line: &str) -> Option<Self> {
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() >= 13 {
            Some(SacctData {
                job_id: fields[0].to_string(),
                job_name: fields[1].to_string(),
                partition: fields[2].to_string(),
                account: fields[3].to_string(),
                alloc_cpus: fields[4].to_string(),
                state: fields[5].to_string(),
                exit_code: fields[6].to_string(),
                start: fields[7].to_string(),
                end: fields[8].to_string(),
                elapsed: fields[9].to_string(),
                time_limit: fields[10].to_string(),
                submit: fields[11].to_string(),
                user: fields[12].to_string(),
                work_dir: fields.get(13).unwrap_or(&"").to_string(),
            })
        } else {
            None
        }
    }

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
        "--format=JobID,JobName,Partition,Account,AllocCPUS,State,ExitCode,Start,End,Elapsed,Timelimit,Submit,User,WorkDir",
        "--parsable2",
        "--noheader",
    ]);

    if let Some(args) = additional_args {
        let arg_parts: Vec<&str> = args.split_whitespace().collect();
        cmd.args(arg_parts);
    }

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("sacct command failed: {}", stderr).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let jobs: Vec<SacctData> = stdout
        .lines()
        .filter_map(SacctData::from_line)
        .collect();

    Ok(jobs)
}