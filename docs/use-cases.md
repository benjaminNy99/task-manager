# Casos de uso

Casos derivados de [docs/requirements.md](requirements.md). Cada archivo contiene la definición completa de un caso de uso.

## Actores y reglas compartidas

- **Usuario:** persona que solicita acceso mediante sus credenciales. Sus operaciones dependen de los roles y permisos de su cuenta.
- **Contribuyente:** puede crear, consultar, modificar y eliminar tareas de proyectos a los que tenga acceso.
- **Líder de proyecto:** dispone de las capacidades del contribuyente y puede crear proyectos, administrar los proyectos a los que tenga acceso, gestionar sus accesos y asignar encargados.
- **Administrador:** dispone de todas las capacidades del líder en cualquier proyecto, sin permiso de acceso explícito, y administra usuarios, incluido bloquear y desbloquear sus cuentas. Esta herencia no exige activar los indicadores de los roles inferiores.
- **Persona que configura el administrador inicial:** puede ejecutar CU-01 sin autenticación previa, únicamente cuando no exista un administrador.

Los actores indicados en el índice incluyen los roles superiores por herencia. El acceso a un proyecto y la autorización de una operación son comprobaciones distintas: una cuenta necesita las capacidades correspondientes además del acceso, salvo el acceso global del administrador.

En todos los casos, los proyectos se representan y seleccionan mediante su sigla, sin distinguir mayúsculas y minúsculas en la selección (RF-12).

### Autenticación y modos de uso

Todos los casos deben estar disponibles mediante comandos y mediante la aplicación interactiva (RF-10). CU-01 incluye el comando de inicialización exigido por RF-26; no se prescribe el mecanismo interactivo para esa misma operación.

Antes de cualquier consulta o persistencia debe haberse ejecutado satisfactoriamente [CU-02](use-cases/CU-02.md). En comandos, esta autenticación se realiza en cada invocación con `--user` y `--pass`. La TUI autentica al entrar y conserva la identidad en memoria. Antes de cada operación, ambos modos comprueban la identidad, el estado de bloqueo y los permisos actuales; una autenticación anterior no permite omitir esas comprobaciones. CU-01 es la única excepción a la autenticación previa. Sus comprobaciones de inicialización forman parte de esa excepción.

Si la autenticación falla, la cuenta está bloqueada o los permisos actuales no permiten la operación, esta no se ejecuta. Estas reglas aplican en ambos modos (RF-15, RF-17, RF-18, RF-23 y RNF-02). Los resultados entregados a CLI y TUI no incluyen contraseñas ni hashes (RNF-09).

### Persistencia y registro de actividad

Los usuarios, proyectos, tareas y permisos se conservan en un archivo SQLite local mediante SeaORM; las ejecuciones que compartan datos utilizan el mismo archivo (RNF-03). Los casos que guardan, editan o eliminan datos incluyen la generación de logs en disco, también si actúa un administrador (RF-09). Los eventos de auditoría se emiten después de confirmar los cambios o conocer el error y los filtros de diagnóstico no los desactivan.

Cada registro incluye fecha y hora, severidad, identificador del usuario, operación, entidad e identificador afectados, resultado y descripción breve; cuando afecta a una tarea incluye también el identificador del proyecto. Los errores se registran con su detalle sin contraseñas ni otros datos sensibles. La generación de logs es comportamiento interno de esos casos, no un caso de uso independiente. Los requerimientos no definen una operación para consultar logs.

Los flujos describen el resultado satisfactorio. Un error de persistencia debe registrarse conforme a RF-09 y no constituye un resultado satisfactorio del caso. Si falla una transacción, se revierten sus cambios (RNF-04). La auditoría es independiente: un fallo del log no revierte la acción realizada. No se especifican reintentos ni otros mecanismos de recuperación.

Las operaciones de datos, el trabajo costoso y la escritura de logs no bloquean el dibujo ni interfieren con la presentación de la TUI (RNF-05). Las fechas se almacenan en UTC y se formatean al mostrarlas (RNF-06).

## Inicialización y acceso

| ID | Nombre | Actor principal | Definición |
|----|--------|-----------------|------------|
| CU-01 | Crear el administrador inicial | Persona que configura el administrador inicial | [CU-01.md](use-cases/CU-01.md) |
| CU-02 | Acceder a la aplicación | Usuario | [CU-02.md](use-cases/CU-02.md) |

## Usuarios

| ID | Nombre | Actor principal | Definición |
|----|--------|-----------------|------------|
| CU-03 | Crear un usuario | Administrador | [CU-03.md](use-cases/CU-03.md) |
| CU-04 | Consultar usuarios | Administrador | [CU-04.md](use-cases/CU-04.md) |
| CU-05 | Modificar un usuario | Administrador | [CU-05.md](use-cases/CU-05.md) |
| CU-06 | Eliminar un usuario | Administrador | [CU-06.md](use-cases/CU-06.md) |

## Proyectos y permisos de acceso

| ID | Nombre | Actor principal | Definición |
|----|--------|-----------------|------------|
| CU-07 | Crear un proyecto | Líder de proyecto | [CU-07.md](use-cases/CU-07.md) |
| CU-08 | Consultar y seleccionar proyectos | Contribuyente | [CU-08.md](use-cases/CU-08.md) |
| CU-09 | Modificar un proyecto | Líder de proyecto | [CU-09.md](use-cases/CU-09.md) |
| CU-10 | Eliminar un proyecto | Líder de proyecto | [CU-10.md](use-cases/CU-10.md) |
| CU-11 | Gestionar el acceso a un proyecto | Líder de proyecto | [CU-11.md](use-cases/CU-11.md) |

## Tareas

| ID | Nombre | Actor principal | Definición |
|----|--------|-----------------|------------|
| CU-12 | Crear una tarea | Contribuyente | [CU-12.md](use-cases/CU-12.md) |
| CU-13 | Consultar las tareas de un proyecto | Contribuyente | [CU-13.md](use-cases/CU-13.md) |
| CU-14 | Modificar una tarea | Contribuyente | [CU-14.md](use-cases/CU-14.md) |
| CU-15 | Gestionar el encargado de una tarea | Líder de proyecto | [CU-15.md](use-cases/CU-15.md) |
| CU-16 | Eliminar una tarea | Contribuyente | [CU-16.md](use-cases/CU-16.md) |

## Cobertura de requerimientos

Esta tabla identifica los casos que materializan cada requerimiento. Las reglas compartidas complementan sus flujos, sin constituir casos adicionales.

| Requerimiento | Casos de uso |
|---------------|--------------|
| RF-01 | CU-12 |
| RF-02 | CU-12, CU-14 |
| RF-03 | CU-12, CU-14 |
| RF-04 | CU-12, CU-13, CU-14, CU-15, CU-16 |
| RF-05 | CU-13, CU-14 |
| RF-06 | CU-14 |
| RF-07 | CU-13, CU-14 |
| RF-08 | CU-13, CU-14 |
| RF-09 | CU-01, CU-03, CU-05, CU-06, CU-07, CU-09, CU-10, CU-11, CU-12, CU-14, CU-15, CU-16 |
| RF-10 | CU-01 a CU-16; regla compartida de modos de uso |
| RF-11 | CU-07 |
| RF-12 | CU-07, CU-08, CU-09 |
| RF-13 | CU-08, CU-13 |
| RF-14 | CU-01, CU-02, CU-03, CU-05 |
| RF-15 | CU-02; autenticación previa en los demás casos, excepto CU-01 |
| RF-16 | CU-03, CU-05; herencia de roles en los casos autorizados |
| RF-17 | CU-08 a CU-16 |
| RF-18 | CU-07 a CU-16 |
| RF-19 | CU-07, CU-08, CU-09, CU-10 |
| RF-20 | CU-12, CU-13, CU-14, CU-16 |
| RF-21 | CU-10 |
| RF-22 | CU-12, CU-15 |
| RF-23 | CU-01, CU-02; autenticación de cada comando |
| RF-24 | CU-01, CU-03, CU-04, CU-05, CU-06 |
| RF-25 | CU-11 |
| RF-26 | CU-01 |
| RF-27 | CU-12, CU-15 |
| RF-28 | CU-12, CU-14, CU-15 |
| RF-29 | CU-05.A1; bloqueo y desbloqueo con autorización y persistencia compartidas con el flujo principal |
| RNF-01 | CU-01, CU-02, CU-03, CU-05 |
| RNF-02 | CU-01, CU-02; condición compartida de consulta y persistencia |
| RNF-03 | Casos de consulta y modificación de usuarios, proyectos, tareas y permisos |
| RNF-04 | Casos que modifican datos; regla compartida de transacciones y auditoría |
| RNF-05 | CU-01 a CU-16 en la TUI; regla compartida de ejecución y presentación |
| RNF-06 | CU-12 y logs de las operaciones que modifican datos |
| RNF-07 | CU-01 a CU-16; implementación compartida por CLI y TUI |
| RNF-08 | Restricción de infraestructura; no representa un objetivo independiente de un actor |
| RNF-09 | CU-01 a CU-16; exclusión de contraseñas y hashes de resultados y de credenciales de logs |
| RNF-10 | Restricción de configuración de infraestructura; no se define un flujo para modificar rutas |

Los 29 requerimientos funcionales están relacionados con casos de uso. El bloqueo y desbloqueo se documentan en CU-05 y el rechazo del borrado por asociaciones de datos en CU-06. Las consideraciones de cada caso identifican decisiones pendientes; no añaden reglas al documento de requerimientos.
