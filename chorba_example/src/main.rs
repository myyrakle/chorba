use chorba::{Decode, Encode};

#[derive(Encode)]
pub struct TestPacket {
    user_id: String,
    user_name: String,
    user_email: String,
}

fn main() {
    println!("Hello, world!");
}
