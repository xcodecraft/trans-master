use std ;
use json ;
pub fn key_f64(jsonobj  : &json::JsonValue,key:&str ) -> f64
{
    let s     = std::fmt::format(format_args!("$v[{}] fail !", key));
    return  jsonobj[key].as_str().expect(s.as_str()).parse().expect("not a number") ;
}

pub fn to_f64(jv : &json::JsonValue) -> f64
{
        match jv{
            &json::JsonValue::String(ref val) => val.as_str().parse().expect("not a number"),
            &json::JsonValue::Number(val) => val.into() ,
            _ => 0.0

        }
}

pub fn idx_f64(jsonobj  : &json::JsonValue,index:usize) -> f64
{
    if jsonobj.is_array()  && !jsonobj.is_empty() {
        match jsonobj {
            &json::JsonValue::Array(ref x) =>
            {
                let data = &x[index] ;
                to_f64(data)
            },
            _ =>  0.0

        }
    }
    else
    {
        0.0
    }

}

pub fn key_u32(jsonobj  : &json::JsonValue,key:&str ) -> u32
{
    return   jsonobj[key].as_u32().expect("convert to u32 failed!");
}
