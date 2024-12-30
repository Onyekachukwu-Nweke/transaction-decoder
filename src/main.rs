use std::io::Read;

fn read_compact_size(transaction_bytes: &mut &[u8]) -> u64 {
    let mut compact_size = [0_u8; 1];
    transaction_bytes.read(&mut compact_size).unwrap();

    if (0..253).contains(&compact_size[0]) {
        compact_size[0] as u64
    } else if compact_size[0] == 253 {
        let mut buffer = [0; 2];
        transaction_bytes.read(&mut buffer).unwrap();
        u16::from_le_bytes(buffer) as u64
    } else if compact_size[0] == 254 {
        let mut buffer = [0; 4];
        transaction_bytes.read(&mut buffer).unwrap();
        u16::from_le_bytes(buffer) as u64
    } else {
        let mut buffer = [0; 8];
        transaction_bytes.read(&mut buffer).unwrap();
        u16::from_le_bytes(buffer) as u64
    }
}

#[allow(unused)]
fn read_version(mut transaction_bytes: &[u8]) -> u32 {
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();
    u32::from_le_bytes(buffer)
}

#[allow(unused_variables)]
fn main() {
    let transaction_hex = "0100000000001c47896df6c74aa8351f371feef54d0b9a90516d74ebe4d0d828";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_version(&mut bytes_slice);

    println!("bytes slice first element: {:?}", bytes_slice[0]);
    println!("version: {}", version);
}

