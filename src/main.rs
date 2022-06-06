use std::io::Write;
use std::io::Read;
use std::process::{Command, Stdio};
use std::{thread, time};
use std::thread::Thread;
use std::env;
use std::process::Child;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
extern crate ini;
use ini::Ini;

struct Worker {
    connection_string: String,
    path: Vec<String>
}

fn main() {
    let remote_cmd: Vec<String> = env::args().skip(1).collect();

    //println!("{}/cliscale.ini", env::args().nth(0).unwrap());

    let worker_conf = Ini::load_from_file(
        format!("{}.ini",
            std::env::current_exe()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
        ))
    .unwrap();

    let mut workers: Vec<Worker> = Vec::new();

    for (sec, prop) in &worker_conf {
        let mut cxn = String::new();
        let mut binpaths = String::new();

        for (key, value) in prop.iter() {
            //println!("value: {}", value);
            match key {
                "connection_string" => {
                    cxn.push_str(value);
                },
                "binpaths" => {
                    binpaths.push_str(value);
                },
                &_ => {}
            }
        }

        workers.push(Worker {
            connection_string: cxn,
            path: binpaths.split(":").map(|s| s.to_string()).collect()
        })
    }

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let mut children: Vec<Child> = Vec::new();
    let (mut tx, rx) = spmc::channel::<String>();

    let mut id = 0;
    for worker in workers {
        let mut child = Command::new("ssh")
            .arg(worker.connection_string)
            .arg(format!("PATH={}", worker.path.join(":")))
            .arg(remote_cmd.join(" ")
        )
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn child process");

        let mut stdin = child.stdin.take().expect("Failed to open SSH stdin");
        children.push(child);
        let rx = rx.clone();
        let thread = thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(received) => {
                        stdin.write(received.as_bytes());
                    }, Err(e) => {
                        break;
                        println!("failed to write to ssh stdin");
                    }
                }
            }
        });

        handles.push(thread);
        id = id + 1;
    }

    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(written) => {
                if written == 0 { // EOF of input stdin reached
                    break;
                }
            },
            Err(e) => {
                println!("Error reading in std:io:stdin");
            }
        }
        
        match tx.send(input) {
            Ok(bytes_maybe) => {
                // no problem
            },
            Err(e) => {
                println!("err on tx: {}", e);
            }
        }
    }

    drop(tx);

    for mut child in children {
        child.wait();
    }
}
