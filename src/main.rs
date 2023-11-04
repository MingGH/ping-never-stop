extern crate ping;

use std::process::Command;
use std::time::Duration;
use job_scheduler::{Job, JobScheduler};
use regex::Regex;
use log::{info, warn};


fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let ip = get_ip().unwrap();

    info!("当前机器ip:{}", ip);

    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/5 * * * * *".parse().unwrap(), || {
        for ip in get_target_ip().iter() {
            let output = Command::new("ping")
                .arg("-c")
                .arg("4")  // 4次ping测试
                .arg(ip)  // 您可以更换为您想要 ping 的地址
                .output()
                .expect("Failed to execute command");
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                info!("{} ", line);
            }
            // info!("{}", format!("{}", String::from_utf8_lossy(&output.stdout)));
        }
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_nanos(100));
    }

}

/***
通过ping命令获取ip
 */
fn get_ip() -> Option<String> {
    let ip_str = Command::new("sh")
        .arg("-c")
        .arg("ifconfig | grep 192.168.193")
        .output()
        .expect("Failed to execute command");

    let re = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();

    if let Some(cap) = re.captures(&*String::from_utf8_lossy(&ip_str.stdout)) {
        return Some(cap[0].to_string());
    }
    None
}


fn get_target_ip() -> [String; 4] {
    // 使用 String 类型定义的数组
    let string_array: [String; 4] = [
        String::from("192.168.193.22"),
        String::from("192.168.193.21"),
        String::from("192.168.193.26"),
        String::from("192.168.193.25"),
    ];
    return string_array;
}