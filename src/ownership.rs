pub fn owner(){
    let s1 = String::from("Hello"); //ownership is with s1
    let s2 = s1; //ownership is transfered to  s2
    println!("{}", s2);
    //Now s1 is no longer valid 
}