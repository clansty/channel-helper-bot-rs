use rand::Rng;

pub fn process_message(test: &str) -> Option<&str> {
    if !test.contains("好不好") {
        return None;
    }
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        Some("好")
    } else {
        Some("不好")
    }
}
