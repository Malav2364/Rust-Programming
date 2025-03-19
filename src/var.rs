pub fn vars(){
    println!("mut is used to make the varaiable mutable");
    let mut a = 10;
    println!("Before {}", a);
    a = 20;
    println!("after {}", a);
    //immutable variables
    println!("immutable variables");
    let b = 50;
    println!("{}", b);
}