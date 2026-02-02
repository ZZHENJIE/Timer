use std::env;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let _ = embed_resource::compile("assets/icon.rc", embed_resource::NONE);
    }
}
