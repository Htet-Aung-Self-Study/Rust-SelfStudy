use std::{process::Command};

fn main() {

    //get the command line result
    let output_result  = Command::new("wmic")
                                .arg("bios")
                                .arg("get")
                                .arg("serialnumber")
                                .output().expect("Something happened!?");
    let commandline_output_result = String::from_utf8_lossy(&output_result.stdout).to_string();
    
    //and read the line 1
    let mut count = 0;
    for line in commandline_output_result.lines() {
        if count == 1 {
            println!("{}", line);
        }
        count += 1;
    }
}
