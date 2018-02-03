extern crate rand ;
extern crate http ;
extern crate curl ;
extern crate json ;
extern crate file ;

//use std::io::{stdin,stdout,Write} ;
use curl::easy::Easy ;
mod xcc_conv ;
mod market ;
use market::{Market,TransTicket,build_ticket} ;


type curl_do = fn(&[u8]) ;
fn  work_ticket(data:&[u8])
{
   let ticket = build_ticket(data,"bitcoin") ;
   Market::instance().receive(ticket) ;
    //println!("data: {} \n {:?} " , std::str::from_utf8(data).unwrap(),ticket) ;
}
fn curl(url : &str , func : curl_do  ) 
{
    let mut easy = Easy::new() ;
    easy.url(url).unwrap();
    easy.write_function( move |data| {
        Ok({
            func(&data) ;
            data.len()
        })
    }).unwrap() ;
    easy.perform().unwrap() ;
} 
fn main() {
    println!("Hello, world!");

    let url = "https://www.bit-z.com/api_v1/ticker?coin=mzc_btc" ;
    let func = work_ticket ;
    curl( &url,func) ;
    curl( &url,func) ;
    curl( &url,func) ;
}
