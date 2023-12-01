use rand::Rng;

pub fn o(zdjd: &str) -> Option<&str> {
    if !zdjd.contains("尊嘟假嘟") {
        return None;
    }
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        Some("尊嘟")
    } else {
        Some("假嘟")
    }
}

#[test]
fn test_zdjd() {
    let result = o("尊嘟假嘟");
    assert!(result == Some("尊嘟") || result == Some("假嘟"));

    let result = o("尊嘟假嘟喵");
    assert!(result == Some("尊嘟") || result == Some("假嘟"));

    let result = o("喵尊嘟假嘟");
    assert!(result == Some("尊嘟") || result == Some("假嘟"));

    let result = o("喵尊嘟假嘟喵");
    assert!(result == Some("尊嘟") || result == Some("假嘟"));

    let result = o("猫");
    assert_eq!(result, None);
}
