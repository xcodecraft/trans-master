extern crate rand ;
extern crate http ;
extern crate curl ;
extern crate json ;
use std::io::{stdin,stdout,Write} ;
use std::rc::Rc ;
use curl::easy::Easy ;
mod xcc_conv ;
mod market ;

use xcc_conv::{to_val,to_u32} ;
use market::{Market,TransTicket,TicketHandler} ;

fn build_ticket(data : &[u8],ckey: &str) -> TicketHandler
{
        let strdata  = std::str::from_utf8(data).unwrap() ;
        let response = json::parse(strdata).unwrap() ;
        let obj      = &response["data"] ;
        TicketHandler::new( TransTicket {
            coin : String::from(ckey),
            date : to_u32(obj,"date"),
            last : to_val(obj,"last"),
            buy  : to_val(obj,"buy"),
            sell : to_val(obj,"sell"),
            high : to_val(obj,"high"),
            low  : to_val(obj,"low"),
            vol  : to_val(obj,"vol"),
        }
        )
}

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
