# Casos de uso

Casos derivados de [docs/requirements.md](requirements.md). Cada archivo contiene la definición completa de un caso de uso.

## Actores y reglas compartidas

- **Usuario:** persona que solicita acceso mediante sus credenciales. Sus operaciones dependen de los roles y permisos de su cuenta.
- **Contribuyente:** puede crear, consultar, modificar y eliminar tareas de proyectos a los que tenga acceso.
- **Líder de proyecto:** dispone de las capacidades del contribuyente y puede crear proyectos, administrar los proyectos a los que tenga acceso, gestionar sus accesos y asignar encargados.
- **Administrador:** dispone de todas las capacidades del líder en cualquier proyecto, sin permiso de acceso explícito, y administra usuarios. Esta herencia no exige activar los indicadores de los roles inferiores.
- **Persona que configura el administrador inicial:** puede ejecutar CU-01 sin autenticación previa, únicamente cuando no exista un administrador.

Los actores indicados en el índice incluyen los roles superiores por herencia. El acceso a un proyecto y la autorización de una operación son comprobaciones distintas: una cuenta necesita las capacidades correspondientes además del acceso, salvo el acceso global del administrador.

En todos los casos, los proyectos se representan y seleccionan mediante su sigla, sin distinguir mayúsculas y minúsculas en la selección (RF-12).

### Autenticación y modos de uso

Todos los casos deben estar disponibles mediante comandos y mediante la aplicación interactiva (RF-10). CU-01 incluye el comando de inicialización exigido por RF-26; no se prescribe el mecanismo interactivo para esa misma operación.

Antes de cualquier consulta o persistencia debe haberse ejecutado satisfactoriamente [CU-02](use-cases/CU-02.md). En comandos, esta autenticación se realiza en cada invocación con `--user` y `--pass`. En el modo interactivo se exige un usuario autenticado antes de cada operación, sin establecer una duración de sesión ni exigir introducir de nuevo las credenciales en cada interacción. CU-01 es la única excepción a la autenticación previa. Sus comprobaciones de inicialización forman parte de esa excepción.

Si la autenticación falla, la operación solicitada no se ejecuta. Cada caso comprueba también los permisos antes de consultar o modificar los datos solicitados. Estas reglas aplican en ambos modos (RF-15, RF-17, RF-18, RF-23 y RNF-02).

### Persistencia y registro de actividad

Los usuarios, proyectos, tareas y permisos se conservan en una base de datos entre ejecuciones (RNF-03). Los casos que guardan, editan o eliminan datos incluyen la generación de logs en disco, también si actúa un administrador (RF-09).

Cada registro incluye fecha y hora, severidad, identificador del usuario, operación, entidad e identificador afectados, resultado y descripción breve; cuando afecta a una tarea incluye también el identificador del proyecto. Los errores se registran con su detalle sin contraseñas ni otros datos sensibles. La generación de logs es comportamiento interno de esos casos, no un caso de uso independiente. Los requerimientos no definen una operación para consultar logs.

Los flujos describen el resultado satisfactorio. Un error de persistencia debe registrarse conforme a RF-09 y no constituye un resultado satisfactorio del caso; no se especifican reintentos, recuperación ni garantías de atomicidad. Tampoco se ha definido la respuesta ante un fallo del propio registro de logs.

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
| RNF-01 | CU-01, CU-03, CU-05 |
| RNF-02 | CU-01, CU-02; condición compartida de consulta y persistencia |
| RNF-03 | Casos de consulta y modificación de usuarios, proyectos, tareas y permisos |

Los 28 requerimientos funcionales tienen cobertura. Las consideraciones de cada caso identifican decisiones pendientes; no añaden reglas al documento de requerimientos.
