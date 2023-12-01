use rand::Rng;

pub fn process_message(message: &str) -> Option<&str> {
    if !message.contains("还是") {
        return None;
    }
    let choices: Vec<&str> = message.split("还是").filter(|s| !s.is_empty()).collect();
    if choices.len() < 2 {
        return None;
    }
    let mut rng = rand::thread_rng();
    Some(choices[rng.gen_range(0..choices.len())])
}

#[test]
fn test() {
    let result = process_message("还是");
    assert_eq!(result, None);

    let result = process_message("1还是");
    assert_eq!(result, None);

    let result = process_message("还是1");
    assert_eq!(result, None);

    let result = process_message("1还是2");
    assert!(result == Some("1") || result == Some("2"));

    let result = process_message("1还是2还是3");
    assert!(result == Some("1") || result == Some("2") || result == Some("3"));
}
