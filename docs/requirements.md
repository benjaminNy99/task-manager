# Requerimientos

Aplicación de gestión de tareas mediante comandos de terminal y una interfaz interactiva en terminal. Este documento consolida los requerimientos acordados.

## Requerimientos funcionales

### Tareas y asignación de encargados

| ID | Requerimiento |
|----|---------------|
| RF-01 | El sistema debe permitir registrar tareas. |
| | • Cada tarea debe incluir nombre, descripción, estado y prioridad. |
| | • El identificador debe asignarse automáticamente y ser un entero positivo incremental. |
| | • La numeración debe ser independiente por proyecto. |
| | • La fecha de ingreso debe asignarse automáticamente por el sistema. |
| RF-02 | El sistema debe admitir únicamente los estados «No empezado», «En curso» y «Completado» para las tareas. |
| RF-03 | El sistema debe admitir únicamente prioridades enteras del 1 al 10, donde 10 representa la máxima urgencia y 1 la mínima. |
| RF-04 | El sistema debe asociar cada tarea a un proyecto. |
| RF-06 | El sistema debe permitir cambiar el estado y la prioridad de las tareas, actualizando su ubicación y orden en pantalla según corresponda. |
| RF-22 | El sistema debe registrar en cada tarea el usuario que la creó y, cuando esté asignado, el usuario encargado de realizarla. |
| RF-27 | El sistema debe permitir gestionar el encargado de las tareas según el rol y el acceso al proyecto. |
| | • Los líderes de proyecto pueden asignar, cambiar o retirar al encargado de las tareas de los proyectos a los que tengan acceso. |
| | • Los administradores pueden realizar esas operaciones en cualquier proyecto. |
| | • El encargado debe ser un usuario con acceso al proyecto, incluido el acceso global de los administradores. |
| | • Solo un líder de proyecto o un administrador puede dejar una tarea sin encargado. |
| RF-28 | El sistema debe asignar automáticamente como encargado al contribuyente que cree una tarea. |
| | • Los contribuyentes que no sean líderes de proyecto ni administradores no deben poder cambiar ni retirar al encargado. |

### Proyectos

| ID | Requerimiento |
|----|---------------|
| RF-11 | El sistema debe permitir crear proyectos. |
| | • Cada proyecto debe tener un identificador numérico positivo incremental, un nombre y una sigla. |
| | • Al crear un proyecto, debe concederse acceso automáticamente a su creador. |
| RF-12 | El sistema debe utilizar la sigla para representar y seleccionar los proyectos. |
| | • Las siglas deben ser únicas, sin espacios ni caracteres especiales. |
| | • Su validación de unicidad y su selección no deben distinguir entre mayúsculas y minúsculas. |
| RF-21 | El sistema debe eliminar todas las tareas de un proyecto cuando se elimine dicho proyecto. |

**Consideraciones pendientes:**

| ID | Consideración |
|----|---------------|
| RF-12 | Falta precisar el conjunto de caracteres admitidos en las siglas, en particular si se permiten letras acentuadas, la letra ñ u otros alfabetos. |

### Usuarios y autenticación

| ID | Requerimiento |
|----|---------------|
| RF-14 | El sistema debe disponer de cuentas de usuario. |
| | • Cada usuario debe tener un identificador numérico positivo incremental, un nombre y una contraseña de acceso. |
| | • Los nombres de usuario deben ser únicos. |
| | • Deben admitir únicamente letras de A a Z, en mayúsculas o minúsculas, y dígitos del 0 al 9. |
| | • No deben admitir espacios, caracteres especiales, acentos, ñ ni otros alfabetos. |
| | • La validación de unicidad y el uso del nombre para identificar al usuario no deben distinguir entre mayúsculas y minúsculas. |
| RF-15 | El sistema debe autenticar al usuario mediante su nombre y contraseña. |
| | • Debe denegar el acceso si la cuenta está bloqueada. |
| | • Los comandos deben autenticar cada invocación. |
| | • La interfaz interactiva debe autenticar al entrar y conservar la identidad en memoria. |
| RF-24 | El sistema debe permitir crear, consultar, modificar y eliminar usuarios. |
| | • Estas operaciones deben estar reservadas a los administradores. |
| | • Se exceptúa la creación del administrador inicial mediante el comando habilitado para ese fin. |
| | • Para eliminar un usuario, debe intentarse el borrado en la base de datos. |
| | • Si la base de datos rechaza el borrado por asociaciones de datos, como una restricción de clave foránea, debe informarse al administrador que no se puede eliminar por estar asociado a otros datos. |
| RF-26 | El sistema debe disponer de un comando para crear el administrador inicial. |
| | • El comando debe estar disponible únicamente cuando no exista un administrador. |
| | • Debe solicitar el nombre y la contraseña de la nueva cuenta. |
| | • Debe establecer la cuenta como administradora sin exigir una autenticación previa. |
| RF-29 | El sistema debe permitir únicamente a los administradores bloquear y desbloquear cuentas de usuario. |

### Roles y permisos

| ID | Requerimiento |
|----|---------------|
| RF-16 | El sistema debe representar los roles de usuario mediante tres campos booleanos y aplicar permisos acumulativos. |
| | • Los campos deben indicar si el usuario es administrador, líder de proyecto y contribuyente. |
| | • El líder de proyecto debe disponer de todas las capacidades del contribuyente. |
| | • El administrador debe disponer de todas las capacidades del líder de proyecto. |
| | • La herencia de capacidades debe aplicarse aunque los indicadores de los roles inferiores estén desactivados. |
| RF-17 | El sistema debe restringir las operaciones de los usuarios no administradores según sus permisos. |
| | • Solo deben tener acceso a los proyectos para los que tengan permiso y a sus tareas. |
| | • Las operaciones disponibles deben ser las permitidas por su rol. |
| | • Estas restricciones deben aplicarse tanto en los comandos como en la aplicación interactiva. |
| RF-18 | El sistema debe permitir a los administradores operar sobre todos los proyectos y sus tareas. |
| | • Deben poder realizar todas las operaciones de los líderes de proyecto. |
| | • No deben necesitar formar parte de los usuarios admitidos en el proyecto. |
| | • Estas capacidades deben aplicarse tanto en los comandos como en la aplicación interactiva. |
| RF-19 | El sistema debe permitir a los líderes de proyecto crear proyectos y consultar, modificar y eliminar los proyectos a los que tengan acceso. |
| RF-20 | El sistema debe permitir a los contribuyentes crear, consultar, modificar y eliminar tareas de los proyectos a los que tengan acceso. |
| RF-25 | El sistema debe permitir conceder y revocar permisos de acceso a los proyectos según el rol. |
| | • Los administradores deben poder gestionar esos permisos en cualquier proyecto. |
| | • Los líderes de proyecto deben poder gestionarlos en los proyectos a los que tengan acceso. |

### Interfaz de terminal y visualización

| ID | Requerimiento |
|----|---------------|
| RF-05 | El sistema debe mostrar las tareas en tres secciones, una por estado. |
| | • Dentro de cada sección, las tareas deben ordenarse de mayor a menor prioridad. |
| | • Las tareas con igual prioridad deben ordenarse por nombre en orden alfabético ascendente. |
| RF-07 | El sistema debe aplicar el color del estado al nombre de la tarea y al nombre del estado. |
| | • Gris para «No empezado». |
| | • Amarillo para «En curso». |
| | • Verde para «Completado». |
| RF-08 | El sistema debe mostrar el número de prioridad en gris para los valores del 1 al 3, amarillo del 4 al 7 y rojo del 8 al 10. |
| RF-10 | El sistema debe ofrecer todas sus operaciones tanto mediante comandos de terminal como mediante la aplicación interactiva en terminal. |
| RF-13 | El sistema debe permitir visualizar varios proyectos y mostrar las tareas de un solo proyecto seleccionado a la vez. |
| RF-23 | El sistema debe autenticar las credenciales de cada comando de terminal antes de ejecutar la operación solicitada. |
| | • Cada comando de la aplicación debe exigir los parámetros `--user` y `--pass`. |
| | • Se exceptúa el comando de creación del administrador inicial, que solo debe estar disponible cuando no exista un administrador. |

### Registro de actividad

| ID | Requerimiento |
|----|---------------|
| RF-09 | El sistema debe generar y almacenar en disco logs de las operaciones de guardado, edición y eliminación de datos. |
| | • Los logs deben generarse después de confirmar los cambios o conocer el error. |
| | • Cada registro debe incluir fecha y hora, severidad, identificador del usuario, operación, entidad afectada y su identificador, resultado y una descripción breve. |
| | • Para las tareas, debe incluir también el identificador del proyecto al que pertenecen. |
| | • Los errores deben registrarse con su detalle, sin exponer contraseñas ni otros datos sensibles. |
| | • La generación de logs debe mantenerse también durante la actividad de los administradores. |
| | • Los filtros de diagnóstico no deben desactivar la generación de logs. |
| | • Un fallo del log no debe revertir la acción realizada. |

## Requerimientos no funcionales

### Seguridad y arquitectura de autenticación

| ID | Requerimiento |
|----|---------------|
| RNF-01 | El sistema debe utilizar Argon2 para generar y verificar los hashes de contraseñas, sin conservar las contraseñas en texto plano. |
| RNF-02 | La arquitectura debe exigir un usuario autenticado para ejecutar cada operación de consulta o persistencia de datos. |
| | • Antes de cada operación, debe comprobar la identidad del usuario, su estado de bloqueo y sus permisos actuales. |
| | • Debe impedir la operación si esas comprobaciones no permiten realizarla. |
| | • Estas condiciones deben aplicarse tanto desde comandos como desde la aplicación interactiva. |
| | • La única excepción debe ser la creación del administrador inicial mediante el comando habilitado cuando no exista un administrador. |
| RNF-09 | El sistema debe excluir contraseñas y hashes de los resultados entregados a CLI y TUI, y excluir las credenciales de los logs. |

### Persistencia

| ID | Requerimiento |
|----|---------------|
| RNF-03 | El sistema debe persistir sus datos en un archivo SQLite local mediante SeaORM. |
| | • Debe almacenar los usuarios, proyectos, tareas y permisos de acceso, conservando sus datos y cambios entre ejecuciones. |
| | • Las ejecuciones que compartan datos deben utilizar el mismo archivo. |
| | • El sistema no debe depender de un servidor central. |
| RNF-04 | El sistema debe ejecutar los cambios agrupados en una transacción como una unidad y revertir los cambios de esa transacción si falla. La auditoría debe ser independiente de la transacción de datos. |
| RNF-08 | El sistema debe mantener cambios versionados del esquema de SQLite mediante migraciones ejecutadas con `sea-orm-migration`. |

### Ejecución y presentación

| ID | Requerimiento |
|----|---------------|
| RNF-05 | El sistema debe ejecutar las operaciones asíncronas mediante Tokio sin bloquear la presentación de la interfaz interactiva. |
| | • Las operaciones de datos no deben bloquear el dibujo de la interfaz ni interferir con su presentación. |
| | • El trabajo costoso no debe bloquear el dibujo de la interfaz ni interferir con su presentación. |
| | • La escritura de logs no debe bloquear el dibujo de la interfaz ni interferir con su presentación. |

### Configuración y fechas

| ID | Requerimiento |
|----|---------------|
| RNF-06 | El sistema debe obtener las fechas del sistema, almacenarlas en UTC mediante `time` y formatearlas al mostrarlas. |
| RNF-10 | El sistema debe utilizar rutas configurables para el archivo SQLite y los archivos de logs. |

### Organización de la aplicación

| ID | Requerimiento |
|----|---------------|
| RNF-07 | El sistema debe implementarse en Rust con un único ejecutable `task-manager` para CLI y TUI. |
| | • CLI y TUI deben utilizar los mismos casos de uso de `task-manager-app`. |
| | • Las reglas de negocio deben residir en `task-manager-core`, sin conocimiento de la terminal ni la base de datos. |
| | • `task-manager-app` debe definir las interfaces de infraestructura y `task-manager-inf` implementarlas. |
| | • `task-manager-inf` debe convertir los modelos y errores de SeaORM a tipos de la aplicación. |
