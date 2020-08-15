mod channel;
mod err;
mod git;
mod shadow;

pub const SHADOW_RS_ENV_PREFIX: &str = "_SHADOW_RUST_ENV_";

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        println!("{}", env!("CARGO_PKG_NAME"));
    }
}
