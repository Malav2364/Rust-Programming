// mod utils;
mod helpers;
mod var;
mod constants;
mod shadowing;
mod ownership;
mod clone;
fn main(){
    constants::constants();
    var::vars();
    shadowing::shadowning();
    ownership::owner();
    clone::clone();    
}