pub mod fancyprinting {
    pub mod ezra {
        pub fn write() {
            println!("ezra");
        }
    }
}

pub use self::fancyprinting::ezra::write as EzraWrite;