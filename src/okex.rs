use std ;
use json ;
use market::{ Market, Unification,TransTicket,TransCell, TransCellVec,ExchangeDept,CoinType} ;
use xcc_conv::{key_f64,idx_f64,key_u32} ;


pub struct OkexApi
{

}
impl Unification for OkexApi
{
    fn fetch_dept(&self) -> Box<ExchangeDept> 
    {
        Box::new(ExchangeDept{coin: CoinType::Btc, 
            asks: Box::new(TransCellVec::new()) , 
            bids : Box::new(TransCellVec::new()) })
    }

    fn name(&self) -> String 
    {
        return String::from("okex") ;

    }
}

impl OkexApi
{

  fn to_ticket(&self,data : &[u8],ckey: &str) -> TransTicket
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

  fn to_dept(&self,data : &[u8],ckey: &str) ->(Box<TransCellVec>,Box<TransCellVec>)
  {
      let strdata  = std::str::from_utf8(data).unwrap() ;
      let jdata    = json::parse(strdata).unwrap() ;
      let jasks    = &jdata["asks"] ;
      let jbids    = &jdata["bids"] ;

      let mut asks = Box::new(TransCellVec::new()) ;
      let mut bids = Box::new(TransCellVec::new()) ;
      match jasks {
        &json::JsonValue::Array(ref x) =>
        {
          for cell in  x{

            asks.push(TransCell{price:idx_f64(&cell,0),vol: idx_f64(&cell,1) }) ;
          }
        },
          _ => ()

      } ;
      return (asks,bids) ;

   }
}

#[cfg(test)]
mod tests {

    use file ;
    use okex::OkexApi ;
    use market::Unification ;
    #[test]
    fn to_ticket() {
    }
    #[test]
    fn to_dept()
    {
            let api        = OkexApi{} ;
            let path        = "./test/okex_dept.json" ;
            let string      = file::get_text(path).expect( path) ;
            let (asks,bids) = api.to_dept(string.as_bytes(),"bitcoin" ) ;
            assert_eq!(string.len(),6638) ;
            assert_eq!(asks.len(),200);
            println!("len : {}" , string.len());

    }
}


