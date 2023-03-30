use rand::Rng;

fn generate_password(length: usize) -> String {
    let mut password = String::new();
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}\\|;:'\",.<>/?";
    let charset_len = charset.len();
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        let index = rng.gen_range(0..charset_len);
        password.push(charset.chars().nth(index).unwrap());
    }

    password
}

fn main() {
    //ask the user for the length of the password
    println!("How long do you want your password to be?");
    let mut length = String::new();
    std::io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line");
    let length: usize = length.trim().parse().expect("Please type a number!");
    //generate the password
    let password = generate_password(length);
    //print the password
    println!("Password: {}", password);
}
