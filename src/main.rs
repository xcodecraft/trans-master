extern crate rand ;
extern crate http ;
extern crate curl ;
extern crate json ;
extern crate file ;
#[macro_use]
extern crate log ;

//use std::io::{stdin,stdout,Write} ;
use curl::easy::Easy ;
mod xcc_conv ;
mod market ;
mod bitz ;
mod okex ;
mod tcurl ;
mod uselog ;


fn main() {
    uselog::init_log() ;
    debug!("program start ....") ;
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {

    use uselog;
    #[test]
    fn do_main()
    {
        uselog::init_log() ;

    }

}


