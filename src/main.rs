use serde_json;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{Read, Write},
    path::Path,
};

#[derive(Debug, Clone, PartialEq)]
enum EstadoTarea {
    Pendiente,
    EnProgreso,
    Completada,
}

impl EstadoTarea {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pendiente" => Some(Self::Pendiente),
            "enprogreso" => Some(Self::EnProgreso),
            "completada" => Some(Self::Completada),
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Pendiente => "Pendiente".to_string(),
            Self::EnProgreso => "En Progreso".to_string(),
            Self::Completada => "Completada".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Tarea {
    id: u32,
    descripcion: String,
    estado: EstadoTarea,
}

impl Tarea {
    fn new(id: u32, descripcion: String) -> Self {
        Self {
            id,
            descripcion,
            estado: EstadoTarea::Pendiente,
        }
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "descripcion": self.descripcion,
            "estado": self.estado.to_string(),
        })
    }

    fn from_json(value: &serde_json::Value) -> Option<Self> {
        let id = value["id"].as_u64()? as u32;
        let descripcion = value["descripcion"].as_str()?.to_string();
        let estado = EstadoTarea::from_str(value["estado"].as_str()?)?;

        Some(Self {
            id,
            descripcion,
            estado,
        })
    }
}

struct GestorTareas {
    tareas: HashMap<u32, Tarea>,
    siguiente_id: u32,
    ruta_archivo: String,
}

impl GestorTareas {
    fn new(ruta_archivo: &str) -> Self {
        Self {
            tareas: HashMap::new(),
            siguiente_id: 1,
            ruta_archivo: ruta_archivo.to_string(),
        }
    }

    fn cargar(&mut self) -> std::io::Result<()> {
        if !Path::new(&self.ruta_archivo).exists() {
            return Ok(());
        }

        let mut archivo = File::open(&self.ruta_archivo)?;
        let mut contenido = String::new();
        archivo.read_to_string(&mut contenido)?;

        let json: serde_json::Value = serde_json::from_str(&contenido)?;
        if let Some(tareas_array) = json.as_array() {
            for tarea_json in tareas_array {
                if let Some(tarea) = Tarea::from_json(tarea_json) {
                    if tarea.id >= self.siguiente_id {
                        self.siguiente_id = tarea.id + 1;
                    }
                    self.tareas.insert(tarea.id, tarea);
                }
            }
        }

        Ok(())
    }

    fn guardar(&self) -> std::io::Result<()> {
        let tareas_json: Vec<serde_json::Value> = self.tareas.values().map(|t| t.to_json()).collect();
        let json_str = serde_json::to_string_pretty(&tareas_json)?;

        let mut archivo = File::create(&self.ruta_archivo)?;
        archivo.write_all(json_str.as_bytes())?;

        Ok(())
    }

    fn agregar_tarea(&mut self, descripcion: String) -> u32 {
        let id = self.siguiente_id;
        self.siguiente_id += 1;
        let tarea = Tarea::new(id, descripcion);
        self.tareas.insert(id, tarea);
        id
    }

    fn actualizar_tarea(&mut self, id: u32, descripcion: String) -> bool {
        if let Some(tarea) = self.tareas.get_mut(&id) {
            tarea.descripcion = descripcion;
            true
        } else {
            false
        }
    }

    fn eliminar_tarea(&mut self, id: u32) -> bool {
        self.tareas.remove(&id).is_some()
    }

    fn cambiar_estado(&mut self, id: u32, estado: EstadoTarea) -> bool {
        if let Some(tarea) = self.tareas.get_mut(&id) {
            tarea.estado = estado;
            true
        } else {
            false
        }
    }

    fn listar_todas(&self) -> Vec<&Tarea> {
        self.tareas.values().collect()
    }

    fn listar_por_estado(&self, estado: EstadoTarea) -> Vec<&Tarea> {
        self.tareas
            .values()
            .filter(|t| matches!(&t.estado, e if *e == estado))
            .collect()
    }
}

fn mostrar_ayuda() {
    println!("Uso:");
    println!("  gestor_tareas agregar <descripción>");
    println!("  gestor_tareas actualizar <id> <nueva_descripción>");
    println!("  gestor_tareas eliminar <id>");
    println!("  gestor_tareas estado <id> <pendiente|enprogreso|completada>");
    println!("  gestor_tareas listar");
    println!("  gestor_tareas listar-pendientes");
    println!("  gestor_tareas listar-enprogreso");
    println!("  gestor_tareas listar-completadas");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        mostrar_ayuda();
        return;
    }

    let mut gestor = GestorTareas::new("tareas.json");
    if let Err(e) = gestor.cargar() {
        eprintln!("Error cargando tareas: {}", e);
        return;
    }

    match args[1].as_str() {
        "agregar" => {
            if args.len() < 3 {
                println!("Error: Falta la descripción de la tarea");
                return;
            }
            let descripcion = args[2..].join(" ");
            let id = gestor.agregar_tarea(descripcion);
            println!("Tarea agregada con ID: {}", id);
        }
        "actualizar" => {
            if args.len() < 4 {
                println!("Error: Falta el ID o la nueva descripción de la tarea");
                return;
            }
            let id = match args[2].parse::<u32>() {
                Ok(id) => id,
                Err(_) => {
                    println!("Error: ID de tarea inválido");
                    return;
                }
            };
            let descripcion = args[3..].join(" ");
            if gestor.actualizar_tarea(id, descripcion) {
                println!("Tarea {} actualizada", id);
            } else {
                println!("Error: No se encontró la tarea {}", id);
            }
        }
        "eliminar" => {
            if args.len() < 3 {
                println!("Error: Falta el ID de la tarea a eliminar");
                return;
            }
            let id = match args[2].parse::<u32>() {
                Ok(id) => id,
                Err(_) => {
                    println!("Error: ID de tarea inválido");
                    return;
                }
            };
            if gestor.eliminar_tarea(id) {
                println!("Tarea {} eliminada", id);
            } else {
                println!("Error: No se encontró la tarea {}", id);
            }
        }
        "estado" => {
            if args.len() < 4 {
                println!("Error: Falta el ID o el estado de la tarea");
                return;
            }
            let id = match args[2].parse::<u32>() {
                Ok(id) => id,
                Err(_) => {
                    println!("Error: ID de tarea inválido");
                    return;
                }
            };
            let estado = match EstadoTarea::from_str(&args[3]) {
                Some(estado) => estado,
                None => {
                    println!("Error: Estado inválido. Usar pendiente, enprogreso o completada");
                    return;
                }
            };
            if gestor.cambiar_estado(id, estado) {
                println!("Estado de la tarea {} cambiado a {}", id, args[3]);
            } else {
                println!("Error: No se encontró la tarea {}", id);
            }
        }
        "listar" => {
            let tareas = gestor.listar_todas();
            if tareas.is_empty() {
                println!("No se encontraron tareas");
            } else {
                println!("Todas las tareas:");
                for tarea in tareas {
                    println!(
                        "{}: {} [{}]",
                        tarea.id,
                        tarea.descripcion,
                        tarea.estado.to_string()
                    );
                }
            }
        }
        "listar-pendientes" => {
            let tareas = gestor.listar_por_estado(EstadoTarea::Pendiente);
            if tareas.is_empty() {
                println!("No hay tareas pendientes");
            } else {
                println!("Tareas pendientes:");
                for tarea in tareas {
                    println!("{}: {}", tarea.id, tarea.descripcion);
                }
            }
        }
        "listar-enprogreso" => {
            let tareas = gestor.listar_por_estado(EstadoTarea::EnProgreso);
            if tareas.is_empty() {
                println!("No hay tareas en progreso");
            } else {
                println!("Tareas en progreso:");
                for tarea in tareas {
                    println!("{}: {}", tarea.id, tarea.descripcion);
                }
            }
        }
        "listar-completadas" => {
            let tareas = gestor.listar_por_estado(EstadoTarea::Completada);
            if tareas.is_empty() {
                println!("No hay tareas completadas");
            } else {
                println!("Tareas completadas:");
                for tarea in tareas {
                    println!("{}: {}", tarea.id, tarea.descripcion);
                }
            }
        }
        _ => {
            println!("Error: Comando desconocido");
            mostrar_ayuda();
            return;
        }
    }

    if let Err(e) = gestor.guardar() {
        eprintln!("Error guardando tareas: {}", e);
    }
}