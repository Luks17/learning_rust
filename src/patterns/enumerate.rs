
// struct with unnamed values using a tuple
pub struct Ipv4Adrr(pub u16, pub u16, pub u16, pub u16);
pub struct Ipv6Addr(pub String);

// types defined by enum can have values inside them in rust, in this example we give structs to them
pub enum IpAddrKind {
  V4(Ipv4Adrr),
  V6(Ipv6Addr),
}

impl Ipv4Adrr {
  pub fn to_string(self: &Self) -> String {
    return format!("{}.{}.{}.{}", self.0, self.1, self.2, self.3);
  }
}

// you can also implement method for an enum in rust
impl IpAddrKind {
  pub fn print_ip_addr(self: &Self) {
    // the match statement is one of rust most important features, unlike if else which only accepts booleans, it can accept enum too 
    // every match 'arm' must represent one possible case of the enum
    match self {
      IpAddrKind::V4(ip) => println!("{}", ip.to_string()),
      IpAddrKind::V6(ip) => println!("{}", ip.0),
    }
  }
}