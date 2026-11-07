use task_manager_core::{AlphanumericNameValidationError, validate_alphanumeric_name};

#[test]
fn alphanumeric_names_accept_ascii_letters_and_digits() {
    for value in ["Proyecto123", "ABC", "abc", "0123456789", "A", "z", "0"] {
        assert_eq!(validate_alphanumeric_name(value), Ok(()), "{value:?}");
    }

    let all_allowed = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    assert_eq!(validate_alphanumeric_name(all_allowed), Ok(()));
}

#[test]
fn alphanumeric_names_reject_empty_values() {
    assert_eq!(
        validate_alphanumeric_name(""),
        Err(AlphanumericNameValidationError::Empty)
    );
}

#[test]
fn alphanumeric_names_reject_non_ascii_and_special_characters() {
    for value in [
        " ",
        " Proyecto",
        "Proyecto ",
        "Mi Proyecto",
        "A\tB",
        "A\nB",
        "A\rB",
        "A-B",
        "A_B",
        "A.B",
        "A/B",
        "A@B",
        "A\0B",
        "á",
        "É",
        "í",
        "Ó",
        "ú",
        "ü",
        "ñ",
        "Ñ",
        "東京",
        "А",
        "١",
        "１",
        "😀",
        "a\u{301}",
        "A\u{a0}B",
    ] {
        assert_eq!(
            validate_alphanumeric_name(value),
            Err(AlphanumericNameValidationError::InvalidCharacters),
            "{value:?}"
        );
    }
}
