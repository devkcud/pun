use std::env::current_exe;

pub fn get_executable_name() -> String {
    let exec_name = current_exe().unwrap();

    let mut exec_name = String::from(exec_name.to_str().unwrap());
    exec_name = exec_name.split("/").last().unwrap().to_string();

    return exec_name;
}
