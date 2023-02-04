use serde::{Serialize, Deserialize};
use uuid::Uuid;

type Status = Vec<JobState>;

#[derive(Serialize, Deserialize)]
pub struct JobState{
    name: String,
    status: JobStatus,
}

#[derive(Serialize, Deserialize)]
pub enum JobStatus{Pending, Ongoing, Fail, Complete}
