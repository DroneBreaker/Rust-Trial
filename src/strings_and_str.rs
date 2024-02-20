pub fn output_str() {
    let name = String::from("Breaker");
    let course = "Rust".to_string();
    let new_name = name.replace("Breaker", "Drone");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name)
}
