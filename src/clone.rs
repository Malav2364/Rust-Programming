pub fn clone(){
    //Cloning Data in Rust
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("Original : {}", s1);
    println!("Cloned : {}", s2);
}