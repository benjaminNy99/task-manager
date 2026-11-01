# Task Manager

Gestor de tareas y proyectos en Rust. El diseño contempla comandos (CLI), una interfaz interactiva de terminal (TUI), usuarios con permisos y almacenamiento en SQLite.

## Estado actual

En desarrollo inicial. Contiene la estructura del proyecto, dependencias y documentación. El ejecutable y las bibliotecas solo incluyen código de ejemplo; las funcionalidades están pendientes.

## Desarrollo

Requiere Rust y Cargo compatibles con la edición 2024 y las dependencias. Ejecuta desde la raíz:

```sh
cargo build --workspace                # Compilar
cargo run                             # Ejecutar el programa de ejemplo
cargo test --workspace                 # Ejecutar pruebas
cargo fmt --all -- --check             # Comprobar formato
cargo clippy --workspace --all-targets # Analizar código
```

## Estructura

| Ruta | Responsabilidad prevista |
| --- | --- |
| `src/` | Ejecutable y presentación CLI/TUI. |
| `crates/task-manager-core/` | Reglas de negocio. |
| `crates/task-manager-app/` | Casos de uso y permisos. |
| `crates/task-manager-inf/` | Persistencia y servicios de infraestructura. |
| `docs/` | Requerimientos, arquitectura y casos de uso. |

## Documentación

- [Requerimientos](docs/requirements.md).
- [Casos de uso](docs/use-cases.md).
- [Arquitectura](docs/architecture.md).
- [Guía para contribuir](AGENTS.md).

## Licencia

[MIT](LICENSE.md).
