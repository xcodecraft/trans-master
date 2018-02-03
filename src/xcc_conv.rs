extern crate json ;
use std ;
pub fn key_f64(jsonobj  : &json::JsonValue,key:&str ) -> f64
{
    let s     = std::fmt::format(format_args!("get json key failed!, {}!", key));
    return  jsonobj[key].as_str().expect(s.as_str()).parse().expect("not a number") ;
}

pub fn idx_f64(jsonobj  : &json::JsonValue,index:usize) -> f64
{
    let s     = std::fmt::format(format_args!("get json key failed!, {}!", index));
    match jsonobj {
        &json::JsonValue::Array(ref x) =>
        {
            let data = &x[index] ;
            return  data.as_str().expect(s.as_str()).parse().expect("not a number") ;

        },
        _ =>  0.0

    }

}

pub fn key_u32(jsonobj  : &json::JsonValue,key:&str ) -> u32
{
    return   jsonobj[key].as_u32().expect("convert to u32 failed!");
}
