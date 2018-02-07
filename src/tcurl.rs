extern crate http ;
extern crate curl ;
use curl::easy::Easy ;
use std::rc::Rc ;

// use syslog ;
// use syslog::{Facility,Severity} ;

pub type VecPtr  = Box<Vec<u8>> ;
pub fn curl(url : &str ) -> VecPtr
{
    // let logger = syslog::unix(Facility::LOG_USER).unwrap() ;
    let mut dataVec = VecPtr::new(Vec::new()) ;
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function( |new_data| {
            dataVec.extend_from_slice(new_data) ;
            debug!("fetch url:{} len:{}",url,new_data.len()) ;
            // logger.send(Severity::LOG_ALERT,"fetch url:{} len:{}";
            Ok(new_data.len())
        }).unwrap() ;

        transfer.perform().unwrap() ;
    }

    return dataVec ;

}
