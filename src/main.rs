extern crate winapi;

use std::io;
use std::io::Write;
use std::process::Command;

use std::ptr::null_mut;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::TokenElevation;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::TOKEN_ELEVATION;

use libc;
use std::mem;
use winapi::ctypes::c_void;
use winapi::um::winnt::TOKEN_QUERY;

struct Windows10 {
    home: &'static str,
    home_n: &'static str,
    pro: &'static str,
    pro_n: &'static str,
}

struct Windows11 {
    pro_11: &'static str,
}

struct Main {
    win10: Windows10,
    win11: Windows11,
    key: Option<String>,
}

impl Main {
    fn new() -> Self {
        Self {
            win10: Windows10 {
                home: "TX9XD-98N7V-6WMQ6-BX7FG-H8Q99",
                home_n: "3KHY7-WNT83-DGQKR-F7HPR-844BM",
                pro: "W269N-WFGWX-YVC9B-4J6C9-T83GX",
                pro_n: "MH37W-N47XK-V7XM9-C7227-GCQG9",
            },
            win11: Windows11 {
                pro_11: "W269N-WFGWX-YVC9B-4J6C9-T83GX",
            },
            key: None,
        }
    }

    fn check_activation() -> bool {
        let output = Command::new("cmd")
            .args(&["/C", "cscript /Nologo C:\\Windows\\System32\\slmgr.vbs /xpr"])
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        output_str.contains("will expire") || output_str.contains("permanently activated.")
    }

    fn platform(&self) -> String {
        let output = Command::new("cmd")
            .args(&["/C", "systeminfo"])
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Extracting OS Name. This is a basic example; in a real-world application, you may need better parsing.
        let re = regex::Regex::new(r"OS Name:\s*(.+)").unwrap();
        re.captures(&output_str).unwrap()[1].to_string()
    }

    fn activate(&mut self) {
        let plat = self.platform();
        println!("[+] Using platform: {}", plat);

        if plat.contains("Microsoft Windows 11") {
            if plat == "Microsoft Windows 11 Pro" {
                self.key = Some(self.win11.pro_11.to_string());
            }
        } else if plat.contains("Microsoft Windows 10") {
            if plat.to_lowercase().contains("home") {
                self.key = Some(self.win10.home.to_string());
            } else if plat.to_lowercase().contains("pro") && !plat.to_lowercase().contains("professional") {
                self.key = Some(self.win10.pro.to_string());
            }
        }

        match &self.key {
            Some(key) => {
                let cmd1 = Command::new("cmd")
                    .args(&["/C", &format!("cscript /Nologo C:\\Windows\\System32\\slmgr.vbs /ipk {}", key)])
                    .output()
                    .expect("Failed to execute command");
                let output1 = String::from_utf8_lossy(&cmd1.stdout);
                if output1.contains("successfully") {
                    println!("[+] Successfully installed product key");
                } else {
                    println!("[-] Error while installing product key");
                }

                let cmd2 = Command::new("cmd")
                    .args(&["/C", "cscript /Nologo C:\\Windows\\System32\\slmgr.vbs /skms kms8.msguides.com"])
                    .output()
                    .expect("Failed to execute command");
                let output2 = String::from_utf8_lossy(&cmd2.stdout);
                if output2.contains("successfully") {
                    println!("[+] KMS set to kms8.msguides.com successfully.");
                } else {
                    println!("[-] Error while setting KMS");
                }

                let cmd3 = Command::new("cmd")
                    .args(&["/C", "cscript /Nologo C:\\Windows\\System32\\slmgr.vbs /ato"])
                    .output()
                    .expect("Failed to execute command");
                let output3 = String::from_utf8_lossy(&cmd3.stdout);
                if output3.contains("successfully") {
                    println!("[+] Product activated successfully");
                    println!("[+] Successfully Activated  :)");
                } else {
                    println!("[-] Error while activating product");
                }
            }
            None => println!("[-] No key available for this platform"),
        }
    }
}
fn is_elevated() -> bool {

    let mut handle: HANDLE = null_mut();
    unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) };

    let elevation = unsafe { libc::malloc(mem::size_of::<TOKEN_ELEVATION>()) as *mut c_void };
    let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
    let mut ret_size = size;
    unsafe {
        GetTokenInformation(
            handle,
            TokenElevation,
            elevation,
            size as u32,
            &mut ret_size,
        )
    };
    let elevation_struct: TOKEN_ELEVATION = unsafe{ *(elevation as *mut TOKEN_ELEVATION)};

    if !handle.is_null() {
        unsafe {
            CloseHandle(handle);
        }
    }

    elevation_struct.TokenIsElevated == 1

}
fn set_window_size(x: i32, y: i32) {
    Command::new("cmd.exe")
        .args(&["/c", &format!("mode con: cols={} lines={}", x, y)])
        .output()
        .expect("Failed to execute command");
}
fn main() {
    
    set_window_size(65, 10);
    println!("[+] Made by @0xHailstorm");
    if !is_elevated() {
        println!("[-] You should run this program as an administrator.");
        let mut reactive = String::new();
        io::stdin().read_line(&mut reactive).expect("Error reading line");
        return;
    }
    if Main::check_activation() {
        println!("[+] Your Windows is Already Active");
        print!("[+] Do You Want to Reactive Windwos? (Y/n): ");
        io::stdout().flush().unwrap();
        let mut reactive = String::new();
        io::stdin().read_line(&mut reactive).unwrap();
        
        if reactive.to_lowercase().contains("n") {
            std::process::exit(0);
        }
    }

    println!("[+] Windows Activating..");

    let mut main_struct = Main::new();
    main_struct.activate();
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Error reading line");
}
