use std::io::{stdin,stdout,Write} ;
use std::rc::Rc ;

#[derive(Debug)]
pub struct TransTicket
{
    pub coin  : String,
    pub date  : u32,
    pub last  : f64,
    pub buy   : f64,
    pub sell  : f64,
    pub high : f64,
    pub low   : f64,
    pub vol   : f64,
} 
pub type  TicketHandler =  Rc<TransTicket> ;
type  TicketHVec    =  Vec<Rc< TransTicket>> ;

pub struct Market
{
    //datas :  Box<TicketHVec>
    datas :  Option<TicketHVec> ,
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
