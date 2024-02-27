enum CryptonVocaloids {
    Meiko(Option<u8>),
    Kaito(Option<u8>),
    HatsuneMiku(Option<u8>),
    MegurineLuka(Option<u8>),
    KagamineRin(Option<u8>),
    KagamineLen(Option<u8>),
}

fn main() {
    println!("Hello, world!");

    // match and if let (powerful control stream keyword) 
    let miku = CryptonVocaloids::HatsuneMiku(Some(16));
    let teto = CryptonVocaloids::HatsuneMiku(Some(31));
    let len = CryptonVocaloids::KagamineLen(Some(14));

    let miku_ret = match miku {
        CryptonVocaloids::HatsuneMiku(Some(16)) => {println!("this is miku"); "miku"}
        CryptonVocaloids::HatsuneMiku(Some(age)) => {println!("this is {} years old miku-like vocaloids", age); "miku-like"}
        _ => {println!("not miku"); "miku plz"}
    };

    let teto_ret = match teto {
        CryptonVocaloids::HatsuneMiku(Some(16)) => {println!("this is miku"); "miku"}
        CryptonVocaloids::HatsuneMiku(Some(age)) => {println!("this is {} years old miku-like vocaloids", age); "miku-like"}
        _ => {println!("not miku"); "miku plz"}
    };

    let len_ret = if let len = CryptonVocaloids::KagamineLen(Some(14)) {
        println!("this is len"); "len"
    } else {
        println!("not len"); "we need kagamine duos"
    };

    println!("miku_ret = {}\nlen_ret = {}", miku_ret, len_ret);
}
