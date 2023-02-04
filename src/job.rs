use crate::env;
use tokio::process;
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::fs::File;
use serde::{Serialize, Deserialize};
//use std::process;
pub struct Job{
    //TODO:maybe refactor this into 1 struct with a name string, step struct and status enum and make a
    //vec of that
    names: Vec<String>,
    steps: Vec<JobStep>,
    status: Vec<JobStatus>,
}
impl Job{
    pub async fn execute_steps(&mut self){
        // zip of a zip is kinda pain ngl
        for ((i,step),name) in self.steps.iter()
            .enumerate()
            .zip(self.names.iter()){
            
            self.status[i] = JobStatus::Ongoing;
            let out_path = {
                let mut tmp = env::get_proc_path()
                    .expect(format!("path error {}",line!()).as_str());
                tmp.push(name);
                tmp.push("stdout.log");
                tmp
            };
            let err_path = {
                let mut tmp = env::get_proc_path()
                    .expect(format!("path error {}",line!()).as_str());
                tmp.push(name);
                tmp.push("stderr.log");
                tmp
            };
            match step.run(std::fs::File::create(out_path).expect("fs error").into(),std::fs::File::create(err_path).expect("fs error").into()){
                Ok(mut child)=>{
                    child.wait().await;
                    self.status[i] = JobStatus::Complete;
                    self.status_update().await;
                },
                Err(e)=>{
                    self.status[i] = JobStatus::Failed;
                    self.status_update().await;
                    eprintln!("{}",e);
                    std::process::exit(1);
                }
            }
            
        }
    }
    pub async fn status_update(&self)->Result<(),std::io::Error>{
        let mut path = env::get_proc_path()?;
        path.push("status.json");
        let serial: Vec<(&String, &JobStatus)> = self.names.iter().zip(self.status.iter()).collect();
        let mut file = File::create(path).await?;
        file.write(serde_json::to_string(&serial)?.as_bytes());
        Ok(())
    }
    pub fn get_status(&self)->Vec<JobStatus>{
        self.status.clone()
    }
}
// expose all the ports fuck it
pub struct JobStep{
    container:String,
    cmd: String,
    env: std::collections::HashMap<String, String>
}
impl JobStep{
    pub fn run(&self, out:std::process::Stdio, err:std::process::Stdio)->Result<process::Child,std::io::Error>{
        let repo_path = {
            let mut tmp =env::get_proc_path()?;
            tmp.push("repo");
            tmp
        };
        process::Command::new("docker")
            .arg("run")
            // attach stdout and  stderr for logs
            .args(["stderr","stdout"].into_iter()
                  .map(|stream|format!("-a {}",stream)))
            // add env args specified
            .args(self.env.iter()
                  .map(|(key,val)|format!("-e {}={}",key,val)))
            //TODO: change this so we read the ports to publish from config instead publishing of all of them
            .arg("-P")
            // mount the cloned repo into /repo in the container
            .args(["-v",format!("{}:{}",
                        repo_path.into_os_string().into_string()
                            .map_err(|e|std::io::Error::new(std::io::ErrorKind::InvalidInput, "wrong"))?,
                        "/repo").as_ref()])
            //set the working directory to where the repo got cloned
            .args(["-w", "/repo"])
            // pipe all stdio to us so we can log it
            .stdout(out)
            .stderr(err)

            .spawn()
    }
}
#[derive(Clone,Serialize, Deserialize)]
pub enum JobStatus{Pending, Ongoing, Complete, Failed}
