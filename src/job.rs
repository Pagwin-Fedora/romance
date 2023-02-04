
use std::process;
pub struct Job{
    step: usize,
    steps: Vec<JobStep>,
    status: Vec<JobStatus>,
    current_proc: process::Child
}

// expose all the ports fuck it
pub struct JobStep{
    container:String,
    cmd: Vec<String>,
    env: std::collections::HashMap<String, String>
}
impl JobStep{
    pub fn run(self)->Result<process::Child,std::io::Error>{
        process::Command::new("docker")
            .arg("run")
            // attach stdin, stdout, stderr for logs
            .args(["stderr","stdout","stdin"].into_iter()
                  .map(|stream|format!("-a {}",stream)))
            // add env args specified
            .args(self.env.into_iter()
                  .map(|(key,val)|format!("-e {}={}",key,val)))
            //TODO: change this so we read the ports to publish from config instead publishing of all of them
            .arg("-P")
            
            .args(["--mount"])
            .spawn()
    }
}
pub enum JobStatus{}
