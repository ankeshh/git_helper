use std::env::args;
use std::process;
fn main() {
    let argument: Vec<String> = args().collect();
    let command = read_args(&argument).unwrap_or_else(|err|{
        print!("Problem passing arguments: {}", err);
        process::exit(1);
    });
    print!("Your entered command is: {:?} \n",command);
    match_comm(&command).unwrap_or_else(|err |{
        print!("Command not found: {}", err);
        process::exit(1);
    });
}

fn read_args(argument: &[String]) -> Result<&str, &str> {
    if argument.len() < 2 {
        return Err("type in the command you want to get help for...");
    }
    let command = &argument[1];
    Ok(command)
}

fn match_comm(command: &str) -> Result<&str, &str> {
    match command {
        "init" => print!("init is used to initalize a repo in a folder."),
        "add" => print!("add is used to stage a any file that has been modified."),
        "clone" => print!("clone is used to copy a repo from a remote system to your local system."),
        "status" => print!("status is used to show modified files in working directory, staged for your next commit."),
        "reset" => print!("reset unstages a file while retaining the changes in working directory."),
        "diff" => print!("diff shows the diff of what is changed but not staged."),
        "commit" => print!("commit is used to commit your staged files and create a snapshot."),
        _ => return Err("enter the correct command"),
    }
    Ok("")
}