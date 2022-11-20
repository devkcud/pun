pub fn show_path() {
    println!("{}", env!("PATH").split(":").collect::<Vec<&str>>().join("\n"));
}
