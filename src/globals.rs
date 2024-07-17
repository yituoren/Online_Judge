use std::sync::Arc;
use tokio::sync::Mutex;
use lazy_static::lazy_static;

use crate::api::job::Job;
use crate::api::user::User;

lazy_static!
{
    pub static ref JOB_LIST: Arc<Mutex<Vec<Job>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref USER_LIST: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
}