use std ;
use json ;
use xcc_conv::{key_f64,idx_f64,key_u32} ;
use std::rc::Rc ;

enum ExchangeKey
{
    Bitz,
    Okc,
}
#[derive(Debug)]
pub struct TransTicket
{
     pub coin  : String,
     pub date  : u32,
     pub last  : f64,
     pub buy   : f64,
     pub sell  : f64,
     pub high  : f64,
     pub low   : f64,
     pub vol   : f64,
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
    data  : Vec< ExcPrice >
}
type  TicketVec     =  Vec< TransTicket> ;
type  PriceVec      =  Vec<ExcPrice> ;

pub struct Market
{
    datas : Option<CoinData>
}

impl Market
{
    pub fn instance() -> &'static mut Market
    {
        unsafe {
        static mut INST: Market = Market{ datas : None };
        if let None = INST.datas  {
            let mut  cdata= CoinData{ start:0 , data : PriceVec::new() };
            INST.datas = Some(cdata) ;
        }
        &mut INST
        }
    }
    pub fn receive(&mut self, ticket :TransTicket )
    {
        //ensure_mut(&self.datas).push(ticket) ;
        //self.datas.as_mut().unwrap().push(ticket) ;
        //let vec : &mut TicketHVec = self.datas.as_mut().unwrap();
        self.datas.as_mut().unwrap().data.push(ExcPrice{key:ExchangeKey::Bitz, price : ticket.last })
       //vec.push(ticket) ;
        //println!("len: {}", vec.len()) ;
    }
}

pub struct TransCell {
    pub price : f64,
    pub vol   : f64
}

pub type  TransCellVec = Vec<TransCell> ;
pub trait Unification {
      fn to_ticket (&self, data : &[u8],ckey: &str) -> TransTicket ;
      fn to_dept(&self,data : &[u8],ckey: &str) -> (Box<TransCellVec>,Box<TransCellVec>) ;
}


#[cfg(test)]
mod tests {
}

