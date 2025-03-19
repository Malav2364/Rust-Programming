pub fn shadowning(){
    println!("Using declared variable with same name with it or other named variable");
    let x = 5;
    let x = x+10;
    let x = x*10;
    println!("x is {}", x);
}