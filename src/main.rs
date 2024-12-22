#[allow(unused_variables)]
fn read_version(transaction_hex: &str) -> u32 {
    return 1;
}

#[allow(unused_variables)]
fn main() {
    let version = read_version("	4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5");
    println!("version: {}", version);
}

