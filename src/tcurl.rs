extern crate http ;
extern crate curl ;
use curl::easy::Easy ;
use std::rc::Rc ;

pub type VecPtr  = Box<Vec<u8>> ;
//pub fn curl(url : &str ) -> Vec<u8> 
pub fn curl(url : &str ) -> VecPtr
{
    let mut dataVec = VecPtr::new(Vec::new()) ;
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function( |new_data| {
            dataVec.extend_from_slice(new_data) ;
            debug!("fetch url:{} len:{}",url,new_data.len()) ;
            Ok(new_data.len())
        }).unwrap() ;

        transfer.perform().unwrap() ;
    }

    return dataVec ;

}
