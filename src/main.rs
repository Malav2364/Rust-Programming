// mod utils;
mod helpers;
mod var;
mod constants;
mod shadowing;
fn main(){
    constants::constants();
    var::vars();
    shadowing::shadowning();    
}