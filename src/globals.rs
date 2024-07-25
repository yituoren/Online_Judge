use std::sync::Arc;
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use rusqlite::Connection;

use crate::api::job::Job;
use crate::api::user::User;
use crate::api::contest::Contest;

//some globals
lazy_static!
{
    pub static ref JOB_LIST: Arc<Mutex<Vec<Job>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref USER_LIST: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref CONTEST_LIST: Arc<Mutex<Vec<Contest>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref DATABASE: Arc<Mutex<Connection>> = Arc::new(Mutex::new(Connection::open("database.db").expect("Failed to open database")));
}