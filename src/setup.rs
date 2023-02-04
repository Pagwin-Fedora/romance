use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use crate::job::{JobStep,Job,JobStatus};
use tokio::io::AsyncReadExt;
use std::process;

pub type JobFile = HashMap<String, PartialJob>;

#[derive(Serialize, Deserialize)]
struct PartialJob{
   pub steps:Vec<JobStep>
}
pub async fn collect_jobs()->Result<Vec<Job>,std::io::Error>{
    let mut file:String = String::new();
    tokio::fs::File::open(".rye_jobs.yaml").await?.read_to_string(&mut file).await;
    // I don't trust serde libs to handle reader correctly
    let config = serde_yaml::from_str::<JobFile>(file.as_str()).expect("yaml parsing error");
    Ok(config.into_iter().map(|(k,v)|Job{
        name:k,
        //the next 2 lines need to be in this exact order due to rust ownership
        status: std::iter::repeat(JobStatus::Pending).take(v.steps.len()).collect(),
        steps:v.steps,
    }).collect())
}

use crate::env::get_repo_path;
pub fn reset_repo()->Result<(),std::io::Error>{
    std::fs::remove_dir_all(get_repo_path()?)?;
    process::Command::new("git")
        .arg("clone")
        .arg(std::env::current_dir()?)
        .arg(get_repo_path()?);
    Ok(())
}
