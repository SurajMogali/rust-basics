



pub enum IpAddrKind{
    V4,
    V6
    
}
pub struct IpAddr{
   pub  kind:IpAddrKind,
   pub  address:String,
}