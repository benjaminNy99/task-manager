#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphanumericNameValidationError {
    Empty,
    InvalidCharacters,
}

impl std::fmt::Display for AlphanumericNameValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(formatter, "El valor no debe estar vacío."),
            Self::InvalidCharacters => {
                write!(
                    formatter,
                    "Solo se admiten letras de A a Z, a a z y dígitos del 0 al 9."
                )
            }
        }
    }
}

impl std::error::Error for AlphanumericNameValidationError {}

/// Valida que el nombre o la sigla no estén vacíos y contengan solo letras ASCII y dígitos.
pub fn validate_alphanumeric_name(value: &str) -> Result<(), AlphanumericNameValidationError> {
    if value.is_empty() {
        return Err(AlphanumericNameValidationError::Empty);
    }

    if !value.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        return Err(AlphanumericNameValidationError::InvalidCharacters);
    }

    Ok(())
}
