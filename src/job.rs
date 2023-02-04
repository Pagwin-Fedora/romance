use crate::env;
use tokio::process;
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::fs::File;
//use std::process;
pub struct Job{
    steps: Vec<JobStep>,
    status: Vec<JobStatus>,
}
impl Job{
    pub async fn execute_steps(&mut self){
        for (step, status) in self.steps.iter_mut().zip(self.status.iter_mut()){
            *status = JobStatus::Ongoing;
            match step.run(){
                Ok(child)=>{
                    let (mut stdout, mut stderr) = (child.stdout.expect("stdout missing"), child.stderr.expect("stderr missing"));
                    tokio::spawn(async move{
                        let log_path = {
                            let mut tmp = env::get_proc_path()
                                .expect(format!("path error {}",line!()).as_str());
                            tmp.push("stdout.log");
                            tmp
                        };
                        let mut log = File::create(log_path).await.expect("fs error");
                        let buf:&mut [u8] = &mut [0;1024];
                        while let Ok(n) = stdout.read(buf).await{
                            // would've preferred putting this in the while loop logic but oh well
                            if n == 0{ break;}
                            log.write(buf).await;
                            log.flush().await;
                        }
                        
                    });
                    tokio::spawn(async move{
                        let log_path = {
                            let mut tmp = env::get_proc_path()
                                .expect(format!("path error {}",line!()).as_str());
                            tmp.push("stderr.log");
                            tmp
                        };
                        let mut log = File::create(log_path).await.expect("fs error");
                        let buf:&mut [u8] = &mut [0;1024];
                        while let Ok(n) = stderr.read(buf).await{
                            // would've preferred putting this in the while loop logic but oh well
                            if n == 0{ break;}
                            log.write(buf).await;
                            log.flush().await;
                        }

                    });
                },
                Err(e)=>{
                    eprintln!("{}",e);
                    std::process::exit(1);
                }
            }
            
        }
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
    pub fn run(&mut self)->Result<process::Child,std::io::Error>{
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
            .args(self.env.into_iter()
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
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdin(std::process::Stdio::piped())

            .spawn()
    }
}
#[derive(Clone)]
pub enum JobStatus{Pending, Ongoing, Complete, Failed}
