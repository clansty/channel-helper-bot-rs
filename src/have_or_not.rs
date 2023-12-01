use rand::Rng;

pub fn process_message(test: &str) -> Option<&str> {
    if !test.contains("有没有") {
        return None;
    }
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        Some("有")
    } else {
        Some("没有")
    }
}
