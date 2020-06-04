use genco::prelude::*;

#[test]
fn test_quote_in() {
    let mut tokens = Tokens::<Rust>::new();
    quote_in!(tokens => fn hello() -> u32 { 42 });
    assert_eq!("fn hello() -> u32 { 42 }", tokens.to_string().unwrap());
}