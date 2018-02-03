//mod xcc_conv ;
use std ;
use json ;
use xcc_conv::{key_f64,idx_f64,key_u32} ;
use std::io::{stdin,stdout,Write} ;
use std::rc::Rc ;

enum ExchangeKey
{
    Bitz,
    Okc,
}
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

pub fn build_ticket(data : &[u8],ckey: &str) -> TransTicket
{
        let strdata  = std::str::from_utf8(data).unwrap() ;
        let response = json::parse(strdata).unwrap() ;
        let obj      = &response["data"] ;
        TransTicket {
            coin : String::from(ckey),
            date : key_u32(obj,"date"),
            last : key_f64(obj,"last"),
            buy  : key_f64(obj,"buy"),
            sell : key_f64(obj,"sell"),
            high : key_f64(obj,"high"),
            low  : key_f64(obj,"low"),
            vol  : key_f64(obj,"vol"),
        }
}
pub struct TransCell {
    price : f64,
    vol   : f64
}

type  TransCellVec = Vec<TransCell> ;
pub fn json_dept(data : &[u8],ckey: &str) ->Box<TransCellVec>
{
        let strdata  = std::str::from_utf8(data).unwrap() ;
        let jobj     = json::parse(strdata).unwrap() ;
        let jdata    = &jobj["data"] ;
        let jasks    = &jdata["asks"] ;
        let jbids    = &jdata["bids"] ;

        let mut asks = Box::new(TransCellVec::new()) ;
        match jasks {
            &json::JsonValue::Array(ref x) =>
            {
                for cell in  x{

                    asks.push(TransCell{price:idx_f64(&cell,0),vol: idx_f64(&cell,1) }) ;
                }
            },
            _ => ()

        } ;
        return asks;


}
#[cfg(test)]
mod tests {

    use market::{Market,TransTicket,build_ticket,json_dept} ;
    use file ;
    #[test]
    fn json_ticket() {
        let json_str= "{\"code\":0,\"msg\":\"Success\",\"data\":{\"date\":1517585637,\"last\":\"0.00085393\", \"buy\":\"0.00085365\",\"sell\":\"0.00085422\",\"high\":\"0.00085421\",\"low\":\"0.00085365\",\"vol\":\"79309.7572\"}}" ;
        let ticket = build_ticket(json_str.as_bytes(),"bitcoin") ;
        assert_eq!(ticket.last, 0.00085393) ;
    }
    #[test]
    fn json_deep()
    {
            let string = file::get_text("./test/depth.json").unwrap() ;
            let asks = json_dept(string.as_bytes(),"bitcoin" ) ;
            assert_eq!(string.len(),4720) ;
            assert_eq!(asks.len(),100);
            println!("len : {}" , string.len());

    }
}

