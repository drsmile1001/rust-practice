pub fn run() {
    let hello = String::from("Здравствуйте");
    let a = hello.chars().take(2).collect::<String>();
    println!("{}", a)
}
