# Repository Guidelines

## Proyecto

Workspace de Rust 2024 en etapa inicial. Revisa `docs/` antes de implementar funcionalidades.

- `src/`: ejecutable y presentación.
- `crates/task-manager-core/`: reglas de negocio.
- `crates/task-manager-app/`: casos de uso.
- `crates/task-manager-inf/`: infraestructura.

Respeta las dependencias: `app` depende de `core`; `inf` depende de ambos.

## Comandos

Desde la raíz:

- `cargo build --workspace`: compilar.
- `cargo run`: ejecutar.
- `cargo test --workspace`: probar.
- `cargo fmt --all -- --check`: comprobar formato.
- `cargo clippy --workspace --all-targets`: analizar código.

## Cambios y estilo

- Revisa la estructura existente y realiza cambios pequeños.
- Conserva nombres, capas y comportamiento salvo que la tarea requiera cambiarlos.
- Usa rustfmt, cuatro espacios, `snake_case` para funciones y `PascalCase` para tipos.
- Evita abstracciones y dependencias innecesarias.

## Pruebas

Las pruebas actuales usan `#[test]` en los archivos `src/lib.rs`. Para lógica nueva, cubre casos válidos y errores con nombres descriptivos. Ejecuta las verificaciones pertinentes e informa cuáles realizaste.

## Commits y pull requests

Usa mensajes breves y claros en español, como en el historial. Describe en cada PR el cambio y su validación; enlaza requerimientos o casos de uso cuando corresponda.

## Seguridad y comunicación

No expongas secretos ni datos sensibles, ni compartas información del proyecto fuera del entorno. Responde en español y resume los archivos modificados, las verificaciones y cualquier limitación.
