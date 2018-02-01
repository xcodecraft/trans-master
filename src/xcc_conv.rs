extern crate json ;
use std ;
pub fn to_val(jsonobj  : &json::JsonValue,key:&str ) -> f64
{
    let s     = std::fmt::format(format_args!("get json key failed!, {}!", key));
    return  jsonobj[key].as_str().expect(s.as_str()).parse().expect("not a number") ;
}
pub fn to_u32(jsonobj  : &json::JsonValue,key:&str ) -> u32
{
    return   jsonobj[key].as_u32().expect("convert to u32 failed!");
}