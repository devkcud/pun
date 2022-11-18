use std::env::current_exe;

pub fn get_exec_name() -> String {
    let exec = current_exe().unwrap();

    let mut exec = String::from(exec.to_str().unwrap());
    exec = exec.split("/").last().unwrap().to_string();

    return exec;
}
