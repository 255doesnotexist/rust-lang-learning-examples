#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("Hello, world!");

    let googledns_v4 = IpAddrKind::V4(8, 8, 8, 8);
    let googledns_v6 = IpAddrKind::V6("2001:4860:4860::8888".to_string());

    route(googledns_v4); route(googledns_v6);
}

fn route(ipaddr: IpAddrKind) {
    match ipaddr {
        IpAddrKind::V4(a, b, c, d) => {
            println!("route ipv4 {}.{}.{}.{}", a, b, c, d);
        }
        IpAddrKind::V6(addr) => {
            println!("route ipv6 {}", addr);
        }
    }
}
