extern crate http ;
extern crate curl ;
use curl::easy::Easy ;


type write_data_fun = fn(&[u8]) ;
fn  work_ticket(data:&[u8])
{

    let api = bitz::BitzApi{} ;
    Market::instance().receive(api) ;
}
fn curl(url : &str , func : write_data_fun)
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
