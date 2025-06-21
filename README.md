# TaskTracker
Tasktracker en rust

Instalación

Clona el repositorio

git clone https://github.com/tu-usuario/gestor-tareas.git
cd gestor-tareas

Compila el proyecto

cargo build --release

Uso
Agregar una nueva tarea
./gestor-tareas agregar "Comprar leche en el supermercado"
Salida:
Tarea agregada con ID: 1

Actualizar una tarea existente
./gestor-tareas actualizar 1 "Comprar leche y pan en el supermercado"
Salida:
Tarea 1 actualizada

Cambiar estado de una tarea
./gestor-tareas estado 1 enprogreso
Salida:
Estado de la tarea 1 cambiado a enprogreso

Listar todas las tareas
./gestor-tareas listar
Salida:
Todas las tareas:
1: Comprar leche y pan en el supermercado [En Progreso]
2: Hacer la presentación del proyecto [Pendiente]

Listar tareas pendientes
./gestor-tareas listar-pendientes

Salida:
Tareas pendientes:
2: Hacer la presentación del proyecto

Listar tareas en progreso
./gestor-tareas listar-enprogreso

Salida:
Tareas en progreso:
1: Comprar leche y pan en el supermercado

Listar tareas completadas
./gestor-tareas listar-completadas
Salida:
No hay tareas completadas

Eliminar una tarea
./gestor-tareas eliminar 2
Salida:
Tarea 2 eliminada

Mostrar ayuda
./gestor-tareas
Salida:
Uso:
  gestor_tareas agregar <descripción>
  gestor_tareas actualizar <id> <nueva_descripción>
  gestor_tareas eliminar <id>
  gestor_tareas estado <id> <pendiente|enprogreso|completada>
  gestor_tareas listar
  gestor_tareas listar-pendientes
  gestor_tareas listar-enprogreso
  gestor_tareas listar-completadas

Almacenamiento de datos
Las tareas se guardan automáticamente en un archivo tareas.json en el mismo directorio donde se ejecuta el programa. El formato es el siguiente:
json
[
  {
    "id": 1,
    "descripcion": "Comprar leche y pan en el supermercado",
    "estado": "En Progreso"
  }
]
