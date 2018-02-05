use std ;
use json ;
use xcc_conv::{key_f64,idx_f64,key_u32} ;

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

pub struct TransCell {
    pub price : f64,
    pub vol   : f64
}

pub type  TransCellVec = Vec<TransCell> ;
pub type  TCellvecPtr  = Box<Vec<TransCell>> ;

pub enum CoinType
{
    Btc  ,
    Ltc  ,
    Eos  ,
    Ustd ,
}
pub struct  ExchangeDept
{
    pub coin : CoinType ,
    pub asks : TCellvecPtr ,
    pub bids : TCellvecPtr ,
}
type DeptPtr = Box<ExchangeDept> ;

pub struct TickDepts
{
    start : u32,
    data  : Vec<DeptPtr>
}
type  TicketVec     =  Vec<TransTicket> ;
type  PriceVec      =  Vec<ExcPrice> ;


pub trait Unification {
        fn name(&self) -> String ;
        fn fetch_dept(&self) -> Box<ExchangeDept> ;
        //fn to_ticket (&self, data : &[u8],ckey: &str) -> TransTicket ;
        //fn to_dept(&self,data : &[u8],ckey: &str) -> (Box<TransCellVec>,Box<TransCellVec>) ;
}

pub struct Market
{
    datas : Option<TickDepts>
}

impl Market
{
    pub fn instance() -> &'static mut Market
    {
        unsafe {
        static mut INST: Market = Market{ datas : None };
        if let None = INST.datas  {
            let mut  cdata= TickDepts{ start:0 , data : Vec::new() };
            INST.datas = Some(cdata) ;
        }
        &mut INST
        }
    }
    pub fn receive<T:Unification>(&mut self, api : T)
    {
        let name = api.name() ;
        let data = api.fetch_dept();

        //api.to_dept
        //ensure_mut(&self.datas).push(ticket) ;
        //self.datas.as_mut().unwrap().push(ticket) ;
        let depts : &mut TickDepts = self.datas.as_mut().unwrap();
        depts.data.push(data )
        //self.datas.as_mut().unwrap().data.push(ExcPrice{key:ExchangeKey::Bitz, price : ticket.last })
       //vec.push(ticket) ;
        //println!("len: {}", vec.len()) ;
    }
}



#[cfg(test)]
mod tests {
}

