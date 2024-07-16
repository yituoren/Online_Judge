use std::sync::Arc;
use tokio::sync::Mutex;
use lazy_static::lazy_static;

use crate::api::job::Job;

lazy_static!
{
    pub static ref JOB_LIST: Arc<Mutex<Vec<Job>>> = Arc::new(Mutex::new(Vec::new()));
}