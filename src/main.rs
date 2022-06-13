use denops_shared_server::config;
use shellexpand::full_with_context_no_errors;
use std::borrow::Cow;
use std::env;
use std::path::PathBuf;

fn main() {
    let f = env::var("DENOPS_SHARED_SERVER_CONFIG")
        .unwrap_or("~/.denops_shared_server/config.json".to_string());
    let f = full_with_context_no_errors(&f, env::home_dir, context);
    println!("Hello, world!");
}

fn context(s: &str) -> Option<Cow<'static, str>> {
    match env::var(s) {
        Ok(value) => Some(value.into()),
        Err(_) => None,
    }
}
