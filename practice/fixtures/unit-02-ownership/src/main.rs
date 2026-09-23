// Intentionally does not compile. Repair the move error without cloning only
// to silence the compiler. Explain which function owns the payload afterward.
fn summarize(payload: String) -> usize {
    payload.len()
}

fn main() {
    let payload = String::from(r#"{"kind":"payment.captured"}"#);
    let length = summarize(payload);
    println!("{length} bytes: {payload}");
}
