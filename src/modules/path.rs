pub fn show_path() {
    let path: Vec<&str> = env!("PATH").split(":").collect();
    println!("{}", path.join("\n"));
}
