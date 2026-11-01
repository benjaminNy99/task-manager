# Requerimientos

Aplicación de gestión de tareas mediante comandos de terminal y una interfaz interactiva en terminal. Este documento consolida los requerimientos acordados.

## Requerimientos funcionales

### Tareas y asignación de encargados

| ID | Requerimiento |
|----|---------------|
| RF-01 | El sistema debe permitir registrar tareas con nombre, descripción, estado y prioridad, asignando automáticamente un identificador entero positivo incremental y la fecha de ingreso. La numeración debe ser independiente por proyecto. |
| RF-02 | El sistema debe admitir únicamente los estados «No empezado», «En curso» y «Completado» para las tareas. |
| RF-03 | El sistema debe admitir únicamente prioridades enteras del 1 al 10, donde 10 representa la máxima urgencia y 1 la mínima. |
| RF-04 | El sistema debe asociar cada tarea a un proyecto. |
| RF-06 | El sistema debe permitir cambiar el estado y la prioridad de las tareas, actualizando su ubicación y orden en pantalla según corresponda. |
| RF-22 | El sistema debe registrar en cada tarea el usuario que la creó y, cuando esté asignado, el usuario encargado de realizarla. |
| RF-27 | El sistema debe permitir a los líderes de proyecto asignar, cambiar o retirar al encargado de las tareas de los proyectos a los que tengan acceso, y a los administradores hacerlo en cualquier proyecto. El encargado debe ser un usuario con acceso al proyecto, incluido el acceso global de los administradores. Solo un líder de proyecto o un administrador puede dejar una tarea sin encargado. |
| RF-28 | El sistema debe asignar automáticamente como encargado al contribuyente que cree una tarea. Los contribuyentes que no sean líderes de proyecto ni administradores no deben poder cambiar ni retirar al encargado. |

### Proyectos

| ID | Requerimiento |
|----|---------------|
| RF-11 | El sistema debe permitir crear proyectos con un identificador numérico positivo incremental, un nombre y una sigla. Al crear un proyecto, debe conceder acceso automáticamente a su creador. |
| RF-12 | El sistema debe utilizar la sigla para representar y seleccionar los proyectos. Las siglas deben ser únicas, sin espacios ni caracteres especiales. Su validación de unicidad y su selección no deben distinguir entre mayúsculas y minúsculas. |
| RF-21 | El sistema debe eliminar todas las tareas de un proyecto cuando se elimine dicho proyecto. |

**Consideraciones pendientes:**

| ID | Consideración |
|----|---------------|
| RF-12 | Falta precisar el conjunto de caracteres admitidos en las siglas, en particular si se permiten letras acentuadas, la letra ñ u otros alfabetos. |

### Usuarios y autenticación

| ID | Requerimiento |
|----|---------------|
| RF-14 | El sistema debe disponer de usuarios con un identificador numérico positivo incremental, un nombre y una contraseña de acceso. Los nombres de usuario deben ser únicos y admitir únicamente letras de A a Z, en mayúsculas o minúsculas, y dígitos del 0 al 9, sin espacios, caracteres especiales, acentos, ñ ni otros alfabetos. Su validación de unicidad y su uso para identificar al usuario no deben distinguir entre mayúsculas y minúsculas. |
| RF-15 | El sistema debe autenticar al usuario mediante su nombre y contraseña y denegar el acceso si su cuenta está bloqueada. Los comandos deben autenticar cada invocación; la interfaz interactiva debe autenticar al entrar y conservar la identidad en memoria. |
| RF-24 | El sistema debe permitir únicamente a los administradores crear, consultar, modificar y eliminar usuarios, salvo la creación del administrador inicial mediante el comando habilitado para ese fin. Para eliminar un usuario, debe intentar el borrado en la base de datos; si esta lo rechaza por asociaciones de datos, como una restricción de clave foránea, debe informar al administrador que no se puede eliminar por estar asociado a otros datos. |
| RF-26 | El sistema debe disponer de un comando para crear el administrador inicial únicamente cuando no exista un administrador. El comando debe solicitar el nombre y la contraseña de la nueva cuenta y establecerla como administradora sin exigir una autenticación previa. |
| RF-29 | El sistema debe permitir únicamente a los administradores bloquear y desbloquear cuentas de usuario. |

### Roles y permisos

| ID | Requerimiento |
|----|---------------|
| RF-16 | El sistema debe representar mediante tres campos booleanos si un usuario es administrador, líder de proyecto y contribuyente. Los permisos deben ser acumulativos: el líder de proyecto debe disponer de todas las capacidades del contribuyente y el administrador de todas las capacidades del líder de proyecto, aunque no tenga activados los indicadores de los roles inferiores. |
| RF-17 | El sistema debe restringir el acceso de los usuarios no administradores a los proyectos para los que tengan permiso y a sus tareas, aplicando las operaciones permitidas por su rol. Esta restricción debe aplicarse tanto en los comandos como en la aplicación interactiva. |
| RF-18 | El sistema debe permitir a los administradores realizar todas las operaciones de los líderes de proyecto sobre cualquier proyecto y sus tareas, sin necesitar formar parte de los usuarios admitidos en el proyecto. Estas capacidades deben aplicarse tanto en los comandos como en la aplicación interactiva. |
| RF-19 | El sistema debe permitir a los líderes de proyecto crear proyectos y consultar, modificar y eliminar los proyectos a los que tengan acceso. |
| RF-20 | El sistema debe permitir a los contribuyentes crear, consultar, modificar y eliminar tareas de los proyectos a los que tengan acceso. |
| RF-25 | El sistema debe permitir a los administradores conceder y revocar permisos de acceso a cualquier proyecto. Los líderes de proyecto deben poder conceder y revocar esos permisos en los proyectos a los que tengan acceso. |

### Interfaz de terminal y visualización

| ID | Requerimiento |
|----|---------------|
| RF-05 | El sistema debe mostrar las tareas en tres secciones, una por estado, ordenadas de mayor a menor prioridad dentro de cada sección. Las tareas con igual prioridad deben ordenarse por nombre en orden alfabético ascendente. |
| RF-07 | El sistema debe aplicar el color del estado al nombre de la tarea y al nombre del estado: gris para «No empezado», amarillo para «En curso» y verde para «Completado». |
| RF-08 | El sistema debe mostrar el número de prioridad en gris para los valores del 1 al 3, amarillo del 4 al 7 y rojo del 8 al 10. |
| RF-10 | El sistema debe ofrecer todas sus operaciones tanto mediante comandos de terminal como mediante la aplicación interactiva en terminal. |
| RF-13 | El sistema debe permitir visualizar varios proyectos y mostrar las tareas de un solo proyecto seleccionado a la vez. |
| RF-23 | El sistema debe exigir los parámetros `--user` y `--pass` en cada comando de terminal de la aplicación y autenticar esas credenciales antes de ejecutar la operación solicitada. Se exceptúa el comando de creación del administrador inicial, que solo debe estar disponible cuando no exista un administrador. |

### Registro de actividad

| ID | Requerimiento |
|----|---------------|
| RF-09 | El sistema debe generar y almacenar en disco logs de las operaciones de guardado, edición y eliminación de datos, después de confirmar los cambios o conocer el error. Cada registro debe incluir fecha y hora, severidad, identificador del usuario, operación, entidad afectada y su identificador, resultado y una descripción breve. Para las tareas, debe incluir también el identificador del proyecto al que pertenecen. Los errores deben registrarse con su detalle, sin exponer contraseñas ni otros datos sensibles. La generación de logs debe mantenerse también durante la actividad de los administradores y no debe desactivarse mediante filtros de diagnóstico. Un fallo del log no debe revertir la acción realizada. |

## Requerimientos no funcionales

### Seguridad y arquitectura de autenticación

| ID | Requerimiento |
|----|---------------|
| RNF-01 | El sistema debe utilizar Argon2 para generar y verificar los hashes de contraseñas, sin conservar las contraseñas en texto plano. |
| RNF-02 | La arquitectura debe exigir un usuario autenticado y comprobar su identidad, estado de bloqueo y permisos actuales antes de ejecutar cada operación de consulta o persistencia de datos, tanto desde comandos como desde la aplicación interactiva. Debe impedir la operación si esas comprobaciones no permiten realizarla. La única excepción debe ser la creación del administrador inicial mediante el comando habilitado cuando no exista un administrador. |
| RNF-09 | El sistema debe excluir contraseñas y hashes de los resultados entregados a CLI y TUI, y excluir las credenciales de los logs. |

### Persistencia

| ID | Requerimiento |
|----|---------------|
| RNF-03 | El sistema debe almacenar los usuarios, proyectos, tareas y permisos de acceso en un archivo SQLite local mediante SeaORM, conservando sus datos y cambios entre ejecuciones. Las ejecuciones que compartan datos deben utilizar el mismo archivo; el sistema no debe depender de un servidor central. |
| RNF-04 | El sistema debe ejecutar los cambios agrupados en una transacción como una unidad y revertir los cambios de esa transacción si falla. La auditoría debe ser independiente de la transacción de datos. |
| RNF-08 | El sistema debe mantener cambios versionados del esquema de SQLite mediante migraciones ejecutadas con `sea-orm-migration`. |

### Ejecución y presentación

| ID | Requerimiento |
|----|---------------|
| RNF-05 | El sistema debe ejecutar las operaciones asíncronas mediante Tokio y realizar las operaciones de datos, el trabajo costoso y la escritura de logs sin bloquear el dibujo de la interfaz interactiva ni interferir con su presentación. |

### Configuración y fechas

| ID | Requerimiento |
|----|---------------|
| RNF-06 | El sistema debe obtener las fechas del sistema, almacenarlas en UTC mediante `time` y formatearlas al mostrarlas. |
| RNF-10 | El sistema debe utilizar rutas configurables para el archivo SQLite y los archivos de logs. |

### Organización de la aplicación

| ID | Requerimiento |
|----|---------------|
| RNF-07 | El sistema debe implementarse en Rust con un único ejecutable `task-manager` para CLI y TUI, que utilicen los mismos casos de uso de `task-manager-app`. Las reglas de negocio deben residir en `task-manager-core`, sin conocimiento de la terminal ni la base de datos. `task-manager-app` debe definir las interfaces de infraestructura y `task-manager-inf` implementarlas, convirtiendo los modelos y errores de SeaORM a tipos de la aplicación. |
