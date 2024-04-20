use std::{fs, io::{self, ErrorKind}, path::Path, process::{Command, Output}};
static DATA_DIRECTORY_NAME: &str = "data";

pub trait CommandExecutor {
    fn execute<'a>(&self, command: &'a str, args: &'a [&'a str]) -> io::Result<Output>;
}

pub struct SystemCommandExecutor;

impl CommandExecutor for SystemCommandExecutor {
    fn execute(&self, command: &str, args: &[&str]) -> io::Result<Output> {
        Command::new(command)
        .args(args)
        .output()
    }
}

/// Retrieves data from the Dukas Copy.
pub fn get_data_from_dukas(executor: &dyn CommandExecutor, symbol: &str, start_date: &str, end_date: &str) -> io::Result<()>{
    let data_dir_path = Path::new(DATA_DIRECTORY_NAME);
    if !data_dir_path.exists() {
        fs::create_dir_all(data_dir_path)?;
        println!("Directory created: {}", DATA_DIRECTORY_NAME);
    }
    
    let formatted_path = format!("{}/{}-tick-{}-{}.csv", DATA_DIRECTORY_NAME, symbol, start_date, end_date);
    let path_data_file = Path::new(&formatted_path);
    
    if path_data_file.exists() {
        println!("{:?} file was find to load data", path_data_file);
        return Ok(());
    }
    
    check_dukas_copy_cli(executor)?;
    println!("Start downloading data...");
    let args = [
        "dukascopy-node", 
        "-i", symbol, 
        "-from", start_date, 
        "-to", end_date, 
        "-t", "tick", 
        "-f", "csv", 
        "--directory", "data"
        ];
    let output = executor.execute("npx", &args)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(ErrorKind::Other, format!("Fail to download data : {}", stderr)));
    }
    println!("Data downloaded successfully.");
    Ok(())
}

fn check_dukas_copy_cli(executor: &dyn CommandExecutor) -> io::Result<()> {
    let node_version_output = executor.execute("node", &["--version"])?;

    if !node_version_output.status.success() {
        let stderr = String::from_utf8_lossy(&node_version_output.stderr);
        return Err(io::Error::new(ErrorKind::Other, format!("Node command failed: {}, please check your node installation", stderr)))
    }

    let install_output = executor.execute("npm", &["install", "dukascopy-node", "--save"])?;

    if !install_output.status.success() {
        let stderr = String::from_utf8_lossy(&install_output.stderr);
        return Err(io::Error::new(ErrorKind::Other, format!("Fail to install dukascopy node package : {}", stderr)))
    }
    Ok(())
}