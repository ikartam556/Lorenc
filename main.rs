fn check_key(key: &str) -> Result<f64, String> {
    if key.is_empty() {
        return Err("Ключ не должен быть пустым".into());
    }
    if !key.bytes().all(|b| (32..=126).contains(&b)) {
        return Err("Ключ должен содержать только печатаемые символы".into());
    }
    let sum: u64 = key.bytes().map(u64::from).sum();
    Ok((sum % 1000) as f64 / 1000.0 + 0.001)
}

fn process(input: &[u8], key: &str, encrypt: bool) -> Result<Vec<u8>, String> {
    let mut x = check_key(key)?;
    Ok(input
        .iter()
        .map(|&b| {
            x = 1.0 - 2.0 * x.abs();
            let k = (((x + 1.0) / 2.0) * 255.0).round() as u8;
            if encrypt { b.wrapping_add(k) } else { b.wrapping_sub(k) }
        })
        .collect())
}

fn main() {
    let message = "фвфвф";
    let key = "0.12133133131";

    let encrypted = process(message.as_bytes(), key, true).unwrap();
    let decrypted = process(&encrypted, key, false).unwrap();

    println!("Зашифровано: {}", String::from_utf8_lossy(&decrypted));
    println!("Расшифровано: {}", String::from_utf8_lossy(&decrypted));
}
