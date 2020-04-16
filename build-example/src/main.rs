include!(concat!(env!("OUT_DIR"), "/commit_hash.rs"));

fn main() {
    println!("Current commit hash id: {}", CURRENT_COMMIT_ID);
}
