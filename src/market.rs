//mod xcc_conv ;
use std ;
use json ;
use xcc_conv::{to_val,to_u32} ;
use std::io::{stdin,stdout,Write} ;
use std::rc::Rc ;

#[derive(Debug)]
pub struct TransTicket
{
     coin  : String,
     date  : u32,
     last  : f64,
     buy   : f64,
     sell  : f64,
     high : f64,
     low   : f64,
     vol   : f64,
} 
enum ExchangeKey
{
    bitz,
    okc,
}
pub struct Exchange
{
    key    : ExchangeKey,
    charge : f64,
} 

struct ExcPrice
{
    key    : ExchangeKey,
    price  : f64
}

pub struct CoinData
{
    start : u32,
    data : Vec< ExcPrice >
}
pub type  TicketHandler =  Rc<TransTicket> ;
type  TicketHVec    =  Vec<Rc< TransTicket>> ;

pub struct Market
{
    //datas :  Box<TicketHVec>
    datas :  Option<TicketHVec> ,
}
struct DataPool
{
    datas : Option<CoinData>
}
impl Market 
{
    pub fn instance() -> &'static mut Market
    {
        unsafe {
        static mut INST: Market = Market{ datas :  None};
        if let None = INST.datas  {
            let mut newVec = TicketHVec::new() ;
            INST.datas = Some( newVec) ;
        } 
        &mut INST
        }
    }
    pub fn receive(&mut self, ticket :TicketHandler )
    {
        //ensure_mut(&self.datas).push(ticket) ;
        //self.datas.as_mut().unwrap().push(ticket) ;
        let vec : &mut TicketHVec = self.datas.as_mut().unwrap();
        vec.push(ticket) ;
        println!("len: {}", vec.len()) ;
    }
}

pub fn build_ticket(data : &[u8],ckey: &str) -> TicketHandler
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