use log::error;

#[derive(PartialEq, Eq, Debug)]
pub enum IpClass {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
    ClassE(String),
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
/// IpClass enum denoting the class of IP.

pub fn classify(ip: (u32, u32, u32, u32)) -> IpClass {
    if (0..=255).contains(&ip.0)
        && (0..=255).contains(&ip.1)
        && (0..=255).contains(&ip.2)
        && (0..=255).contains(&ip.3)
    {
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
            (x, _, _, y) if (240..=255).contains(&x) && (0..255).contains(&y) => {
                IpClass::ClassE(format!("{}.{}.{}.{}", ip.0, ip.1, ip.2, ip.3))
            }
            _ => IpClass::None,
        }
    } else {
        error!("Wrong IP");
        IpClass::None
    }
}
