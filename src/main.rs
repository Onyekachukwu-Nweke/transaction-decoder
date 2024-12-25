#[allow(unused)]
fn read_version(transaction_hex: &str) -> u32 {
    let tx_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes: [u8; 4] = (&tx_bytes[0..4]).try_into().unwrap();
    u32::from_le_bytes(version_bytes)
}

#[allow(unused_variables)]
fn main() {
    let version = read_version("0100000000001c47896df6c74aa8351f371feef54d0b9a90516d74ebe4d0d828");
    println!("version: {}", version);
}

