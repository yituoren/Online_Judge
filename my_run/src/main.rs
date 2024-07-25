use tokio::{task, self};
use tokio::time::{timeout, Duration};
use tokio::process::Command;
use std::process::Stdio;
use tokio::fs::File;
use libc::{wait4, rusage, setrlimit, RLIMIT_AS, rlimit};
use clap::{Arg, self};

async fn run_case(path: &str, in_file: File, out_file: File, time_limit: Duration, memory_limit: u64, memory: &mut u64) -> Option<i32>
{
    //limit memory
    if memory_limit != 0
    {
        let _ = set_memory_limit(memory_limit);
    }

    //run the code
    let mut child = Command::new(path.to_string() + "main")
        .stdin(Stdio::from(in_file.into_std().await))
        .stdout(Stdio::from(out_file.into_std().await))
        .stderr(Stdio::null())
        .spawn();

    let mut pid: i32 = 0;
    //MLE
    if child.is_err()
    {
        println!("-1");
        return None;
    }
    else
    {
        pid = child.as_ref().unwrap().id().unwrap() as i32;
    }

    match timeout(time_limit, waitfor(pid, memory)).await
    {
        Ok(result) =>
        {
            Some(result)
        }
        Err(_) => //TLE
        {
            let _ = child.unwrap().kill().await;
            None
        }
    }
}

fn set_memory_limit(memory_limit: u64) -> std::io::Result<()> {
    let rlim = rlimit {
        rlim_cur: memory_limit,
        rlim_max: memory_limit,
    };

    unsafe
    {
        if setrlimit(RLIMIT_AS, &rlim) != 0
        {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok(())
}

//this function aims to change wait4() to an async function and make it non-blocking
async fn waitfor(pid: i32, memory: &mut u64) -> i32
{
    let (result, maxrss) = task::spawn_blocking(move || {
        let mut status: i32 = 0;
        let mut usage = unsafe { std::mem::zeroed::<rusage>() };
        unsafe { wait4(pid, &mut status, 0, &mut usage); }
        (status, usage.ru_maxrss)
    }).await.unwrap();
    *memory = maxrss as u64 * 1024;
    result
}

#[tokio::main]
async fn main()
{
    //ask for command line
    let args = clap::Command::new("OJ")
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .value_name("PATH")
            .required(true))
        .arg(Arg::new("in_file")
            .short('i')
            .long("in")
            .value_name("IN_FILE")
            .required(true))
        .arg(Arg::new("out_file")
            .short('o')
            .long("out")
            .value_name("OUT_FILE")
            .required(true))
        .arg(Arg::new("time_limit")
            .short('t')
            .long("time")
            .value_name("TIME_LIMIT")
            .required(true))
        .arg(Arg::new("memory_limit")
            .short('m')
            .long("memory")
            .value_name("MEMORY_LIMIT")
            .required(true))
        .get_matches();

    let path = args.get_one::<String>("path").unwrap();
    let in_file = File::open(args.get_one::<String>("in_file").unwrap()).await.unwrap();
    let out_file = File::create(args.get_one::<String>("out_file").unwrap()).await.unwrap();
    let time_limit = Duration::from_micros(args.get_one::<String>("time_limit").unwrap().parse::<u64>().unwrap());
    let memory_limit = args.get_one::<String>("memory_limit").unwrap().parse::<u64>().unwrap();
    let mut memory: u64 = 0;

    match run_case(path, in_file, out_file, time_limit, memory_limit, &mut memory).await
    {
        //pass the status
        Some(status) =>
        {
            println!("{}", status);
            println!("{}", memory);
        }
        None => (),
    }
}