use pass_generator::password::generate_password;

#[test]
fn test_password_length() {
    let charset = b"abc";
    let p = generate_password(10, charset);
    assert_eq!(p.len(), 10);
}

#[test]
fn test_password_uses_charset() {
    let charset = b"abc";
    let p = generate_password(50, charset);

    for c in p.chars() {
        assert!(charset.contains(&(c as u8)));
    }
}

#[test]
fn test_password_with_single_char_charset() {
    let charset = b"x";
    let p = generate_password(20, charset);
    assert_eq!(p, "xxxxxxxxxxxxxxxxxxxx");
}
