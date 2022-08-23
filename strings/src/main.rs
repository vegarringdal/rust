fn borrow_print(s1: &str, s2: &str, s3: &str) {
    println!("");
    println!("S1: {}", s1);
    println!("S2: {}", s2);
    println!("S3: {}", s3);
    println!("");
}

fn new_owner(s1: String, s2: String, s3: String) -> (String, String, String) {
    println!("");
    println!("S1: {}", s1);
    println!("S2: {}", s2);
    println!("S3: {}", s3);
    println!("");

    (s1, s2, s3)
}

fn main() {
    let my_string1 = String::from("something");
    let my_string2 = "something else".to_owned();
    let my_string3 = "what else ".to_string();

    borrow_print(&my_string1, my_string2.as_str(), my_string3.as_ref());
    borrow_print(&my_string1, &my_string2, &my_string3);

    let (mut my_string1, my_string2, my_string3) = new_owner(my_string1, my_string2, my_string3);

    my_string1.push_str("WOOOOOOOOOOOOOOWO, I can actually edit this");
    let my_string1 = my_string1; // not its not mutable anymore..

    borrow_print(&my_string1, &my_string2, &my_string3);

    println!("to_uppercase: {}", my_string1.to_uppercase());
    let my_vec: Vec<&str> = my_string1.split(' ').collect();
    println!("to_uppercase: {:?}", my_vec);
    let my_vec: Vec<&str> = my_vec.iter().map(|&x| x.to_uppercase()).collect(); // why?
    println!("to_uppercase: {:?}", my_vec);
}
