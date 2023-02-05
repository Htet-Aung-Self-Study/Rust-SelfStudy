extern crate winreg;

use std::{process::Command};
use std::path::Path;

use winreg::enums::*;
use winreg::RegKey;

const company: &str = "Company";

fn main() {

    //get the command line result
    let output_result  = Command::new("wmic")
                                .arg("bios")
                                .arg("get")
                                .arg("serialnumber")
                                .output().expect("Something happened!?");
    let commandline_output_result = String::from_utf8_lossy(&output_result.stdout).to_string();
    
    //and read the line 1
    let mut serialnumber: String = "".into();
    let mut count = 0;
    for line in commandline_output_result.lines() {
        if count == 1 {
            serialnumber = line.into();
            println!("{}", line);
        }
        count += 1;
    }

    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software").join(company);
    let (key, _disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("ID", &serialnumber);
}
