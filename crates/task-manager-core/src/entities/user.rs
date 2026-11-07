use std::fmt;
use std::num::NonZeroU64;

/// Cuenta de usuario. Los indicadores de rol conservan los valores proporcionados.
#[derive(Debug)]
pub struct User {
    pub id: NonZeroU64,
    pub name: String,
    pub password_hash: PasswordHash,
    pub is_blocked: bool,
    pub is_admin: bool,
    pub is_project_leader: bool,
    pub is_contributor: bool,
}

/// Hash de contraseña preparado por infraestructura; no contiene la contraseña original.
pub struct PasswordHash(String);

impl PasswordHash {
    /// Recibe un hash ya generado o recuperado de persistencia.
    ///
    /// No genera ni valida el hash. El llamador debe proporcionar un hash Argon2 válido,
    /// nunca una contraseña en texto plano.
    pub fn from_encoded_hash(encoded_hash: String) -> Self {
        Self(encoded_hash)
    }

    /// Acceso explícito al hash para persistencia y verificación; no usar en logs o respuestas.
    pub fn as_encoded_hash(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for PasswordHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PasswordHash([REDACTED])")
    }
}
