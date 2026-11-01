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
| RF-14 | El sistema debe disponer de usuarios con un identificador numérico positivo incremental, un nombre y una contraseña de acceso. Los nombres de usuario deben ser únicos, sin espacios ni caracteres especiales. Su validación de unicidad y su uso para identificar al usuario no deben distinguir entre mayúsculas y minúsculas. |
| RF-15 | El sistema debe autenticar al usuario mediante su nombre y contraseña para permitir el acceso a la aplicación. |
| RF-24 | El sistema debe permitir únicamente a los administradores crear, consultar, modificar y eliminar usuarios, salvo la creación del administrador inicial mediante el comando habilitado para ese fin. |
| RF-26 | El sistema debe disponer de un comando para crear el administrador inicial únicamente cuando no exista un administrador. El comando debe solicitar el nombre y la contraseña de la nueva cuenta y establecerla como administradora sin exigir una autenticación previa. |

**Consideraciones pendientes:**

| ID | Consideración |
|----|---------------|
| RF-14 | Falta precisar el conjunto de caracteres admitidos en los nombres de usuario, en particular si se permiten letras acentuadas, la letra ñ u otros alfabetos. |

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
| RF-09 | El sistema debe generar y almacenar en disco logs de las operaciones de guardado, edición y eliminación de datos. Cada registro debe incluir fecha y hora, severidad, identificador del usuario, operación, entidad afectada y su identificador, resultado y una descripción breve. Para las tareas, debe incluir también el identificador del proyecto al que pertenecen. Los errores deben registrarse con su detalle, sin exponer contraseñas ni otros datos sensibles. La generación de logs debe mantenerse también durante la actividad de los administradores. |

## Requerimientos no funcionales

### Seguridad y arquitectura de autenticación

| ID | Requerimiento |
|----|---------------|
| RNF-01 | El sistema debe almacenar las contraseñas mediante un hash seguro para contraseñas, sin conservarlas en texto plano. |
| RNF-02 | La arquitectura debe exigir un usuario autenticado como condición previa para ejecutar cualquier operación de consulta o persistencia de datos, tanto desde comandos como desde la aplicación interactiva. La única excepción debe ser la creación del administrador inicial mediante el comando habilitado cuando no exista un administrador. |

### Persistencia

| ID | Requerimiento |
|----|---------------|
| RNF-03 | El sistema debe almacenar en una base de datos los usuarios, proyectos, tareas y permisos de acceso, conservando sus datos y cambios entre ejecuciones. |
