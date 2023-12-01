use rand::Rng;

pub fn process_message(test: &str) -> Option<&str> {
    if !test.contains("是不是") {
        return None;
    }
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        Some("是")
    } else {
        Some("不是")
    }
}
