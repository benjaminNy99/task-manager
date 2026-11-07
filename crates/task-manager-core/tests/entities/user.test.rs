use std::num::NonZeroU64;

use task_manager_core::{PasswordHash, User};

// Valor ficticio para probar el modelo; estas pruebas no generan ni verifican hashes.
const TEST_HASH: &str = "hash-ficticio-de-prueba";

fn user() -> User {
    User {
        id: NonZeroU64::new(1).unwrap(),
        name: "Usuario1".to_owned(),
        password_hash: PasswordHash::from_encoded_hash(TEST_HASH.to_owned()),
        is_blocked: false,
        is_admin: false,
        is_project_leader: false,
        is_contributor: false,
    }
}

#[test]
fn user_preserves_identity_and_encoded_password_hash() {
    let user = user();

    assert_eq!(user.id.get(), 1);
    assert_eq!(user.name, "Usuario1");
    assert_eq!(user.password_hash.as_encoded_hash(), TEST_HASH);
}

#[test]
fn user_preserves_each_combination_of_role_indicators() {
    for is_admin in [false, true] {
        for is_project_leader in [false, true] {
            for is_contributor in [false, true] {
                let user = User {
                    is_admin,
                    is_project_leader,
                    is_contributor,
                    ..user()
                };

                assert_eq!(user.is_admin, is_admin);
                assert_eq!(user.is_project_leader, is_project_leader);
                assert_eq!(user.is_contributor, is_contributor);
            }
        }
    }
}

#[test]
fn user_can_represent_blocked_and_unblocked_accounts() {
    let mut user = user();

    assert!(!user.is_blocked);
    user.is_blocked = true;
    assert!(user.is_blocked);
    user.is_blocked = false;
    assert!(!user.is_blocked);
}

#[test]
fn debug_hides_password_hash_including_inside_user() {
    let user = user();

    assert_eq!(
        format!("{:?}", user.password_hash),
        "PasswordHash([REDACTED])"
    );
    let debug = format!("{user:?}");
    assert!(debug.contains("PasswordHash([REDACTED])"));
    assert!(!debug.contains(TEST_HASH));
}
