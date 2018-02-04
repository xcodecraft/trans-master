use std ;
use json ;
use market::{ Market, Unification,TransTicket,TransCell, TransCellVec} ;
use xcc_conv::{key_f64,idx_f64,key_u32} ;


pub struct BitzApi
{

}
impl Unification for BitzApi
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
      let jobj     = json::parse(strdata).unwrap() ;
      let jdata    = &jobj["data"] ;
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
    use bitz::BitzApi ;
    use market::Unification ;
    #[test]
    fn json_ticket() {
        let json_str= "{\"code\":0,\"msg\":\"Success\",\"data\":{\"date\":1517585637,\"last\":\"0.00085393\", \"buy\":\"0.00085365\",\"sell\":\"0.00085422\",\"high\":\"0.00085421\",\"low\":\"0.00085365\",\"vol\":\"79309.7572\"}}" ;
        let bapi   = BitzApi{} ;
        let ticket = bapi.to_ticket(json_str.as_bytes(),"bitcoin") ;
        assert_eq!(ticket.last, 0.00085393) ;
    }
    #[test]
    fn json_deep()
    {
            let bapi        = BitzApi{} ;
            let path        = "./test/bitz_dept.json" ;
            let string      = file::get_text(path).expect( path) ;
            let (asks,bids) = bapi.to_dept(string.as_bytes(),"bitcoin" ) ;
            assert_eq!(string.len(),4720) ;
            assert_eq!(asks.len(),100);
            println!("len : {}" , string.len());

    }
}


