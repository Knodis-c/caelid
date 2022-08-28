use std::{
    io::{self, Write},
    fs::{OpenOptions, self},
    process,
};

pub const PID_FILE_NAME: &'static str = "server.pid";

pub fn create() -> Result<u32, io::Error> {
    let maybe_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(PID_FILE_NAME);

    match maybe_file {
        Ok(mut f) => {
            let pid = process::id();
            f.write(format!["{}", pid].as_bytes())?;
            log::info!("PID: {pid}");
            Ok(pid)
        },
        Err(e) => if let io::ErrorKind::AlreadyExists = e.kind() {
            let existing_pid = fs::read_to_string(PID_FILE_NAME)?;
            log::error!("There's an ongoing process with ID: {existing_pid}. If incorrect, remove server.pid from project root and retry.");
            process::exit(1);
        } else {
            Err(e)
        }
    }
}

pub fn destroy() -> Result<(), io::Error> {
    fs::remove_file(PID_FILE_NAME)
}
