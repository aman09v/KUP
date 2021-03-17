#[derive(PartialEq, Eq, Debug)]
pub enum IpClass {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
    None,
}

/// classify function classifies ip addresses into classes.
///
/// #Arguments
///
/// ip - A tuple denoting the octets of ip address.
///
/// #Return
///
/// enum type (IpClass).

pub fn classify(ip: (u32, u32, u32, u32)) -> IpClass {
    match ip {
        (x, _, _, _) if (0..=127).contains(&x) => {
            IpClass::ClassA(format!("{}.{}.{}.{}", ip.0, ip.1, ip.2, ip.3))
        }
        (x, _, _, _) if (128..=191).contains(&x) => {
            IpClass::ClassB(format!("{}.{}.{}.{}", ip.0, ip.1, ip.2, ip.3))
        }
        (x, _, _, _) if (192..=223).contains(&x) => {
            IpClass::ClassC(format!("{}.{}.{}.{}", ip.0, ip.1, ip.2, ip.3))
        }
        (x, _, _, _) if (224..=239).contains(&x) => {
            IpClass::ClassD(format!("{}.{}.{}.{}", ip.0, ip.1, ip.2, ip.3))
        }
        _ => IpClass::None,
    }
}
