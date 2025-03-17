
pub trait DiscordTypes {
    fn from(value: u8) -> Self;
    fn value(&self) -> u8;
}

pub trait DiscordFlagsTypes {
    fn from(value: i32) -> Self;
    fn value(&self) -> i32;
}

pub trait DiscordStringTypes {
    fn from(value: String) -> Self;
    fn value(&self) -> String;
}