use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use crate::job::{JobStep,Job,JobStatus};
use tokio::io::AsyncReadExt;
use std::process;
use crate::env::get_repo_path;

pub type JobFile = HashMap<String, PartialJob>;

#[derive(Serialize, Deserialize)]
pub struct PartialJob{
   pub steps:Vec<JobStep>
}
pub async fn collect_jobs()->Result<Vec<Job>,std::io::Error>{
    let mut file:String = String::new();
    let path = {
        let mut tmp = get_repo_path()?;
        tmp.push(".romance_jobs.yaml");
        tmp
    };
    tokio::fs::File::open(path).await?.read_to_string(&mut file).await?;
    // I don't trust serde libs to handle reader correctly
    let config = serde_yaml::from_str::<JobFile>(file.as_str()).expect("yaml parsing error");
    Ok(config.into_iter().map(|(k,v)|Job{
        name:k,
        //the next 2 lines need to be in this exact order due to rust ownership
        status: std::iter::repeat(JobStatus::Pending).take(v.steps.len()).collect(),
        steps:v.steps,
    }).collect())
}

pub fn reset_repo()->Result<(),std::io::Error>{
    std::fs::remove_dir_all(get_repo_path()?)?;
    process::Command::new("git")
        .arg("clone")
        .arg(std::env::current_dir()?)
        .arg(get_repo_path()?)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?.wait()?;
    Ok(())
}
pub fn setup_dirs()->std::io::Result<()>{
    use std::fs;
    use crate::env;
    let mut b = fs::DirBuilder::new();
    b.recursive(true);
    b.create(env::get_repo_path()?)?;
    b.create(env::get_proc_path()?)?;

    Ok(())
}
