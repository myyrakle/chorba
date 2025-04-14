use chorba::{Decode, Decoder, Encode, decode, encode};

#[derive(Encode, Decode, Debug)]
pub struct TestPacket {
    user_id: String,
    user_name: String,
    user_email: String,
}

fn main() {
    let encoded = encode(&TestPacket {
        user_id: "123".to_string(),
        user_name: "John Doe".to_string(),
        user_email: "myyrakle@naver.com".to_string(),
    });

    println!("encoded: {:?}", encoded);

    let decoded: TestPacket = decode(&encoded).unwrap();
    println!("decoded: {:?}", decoded);
}
