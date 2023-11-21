fn main() {
    let url = std::env::var("QR_SERVER_URL").unwrap_or("http://127.0.0.1:3000/qr".to_string());
    let key = std::env::var("QR_SERVER_KEY").unwrap_or("qr_server".to_string());
    let args: Vec<String> = std::env::args().collect();
    for q in &args[1..] {
        let b = q.as_bytes();
        let k = key.as_bytes();
        let k = [b, k];
        let d = md5::compute(k.concat());
        let d = format!("{}?{:x}{}", url, d, urlencoding::encode(q));
        println!("{}", d);
    }
}
