use std::io;

struct Task {
    name : String,
    description : String,
    completed : bool,
}

impl Task {
    fn add_task(tasks : &mut Vec<Task>,name : String,  description : String) {
             let task = Task {
                name : name.trim().to_string(),
                description : description.trim().to_string(),
                completed : false,
            };
            tasks.push(task);
            println!("\nTarea Agregada!!|\n");
    }

    fn list_tasks(tasks : & Vec<Task>) {
        println!("\n======Tareas : {}=====", tasks.len());
        for task in tasks {
            println!("Nombre: {}", task.name);
            println!("Descripcion: {}", task.description);
            if task.completed {
                println!("Estado: Completada\n");
            } else {
                println!("Estado: No Completada\n");
            }
        }
    }

    fn completed_task(tasks : &mut Vec<Task>, name : String) {
        match Self::find_task(&name, tasks) {
            Some(task) => {
                task.completed = true;
                println!("\nEstado de tarea cambiado!!\n");
            },
            None => println!("\nTarea no encontrada\n"),
        }
    }

    fn find_task<'a>(name : &str, tasks : &'a mut  Vec<Task>) ->Option<&'a mut Task> {
        for task in tasks {
            if name == task.name {
                return Some(task);
            }
        }
        None
    }
}

fn main() {
    let mut tasks : Vec<Task>  = Vec::new();

    loop {
        println!("========Menu========");
        println!("1. Listar Tareas");
        println!("2. Agregar Tarea");
        println!("3. Completar Tarea");
        println!("4. Salir");

        let mut option = String::new();
        println!("Digite una opcion: ");
        io::stdin().read_line(&mut option).unwrap();
        
        match option.trim() {
            "1" => Task::list_tasks(& tasks),
            "2" => {
                let mut name = String::new();
                println!("\nIngrese el nombre de la Tarea: ");
                io::stdin().read_line(&mut name).unwrap();

                let task = Task::find_task(&name.trim().to_string(), &mut tasks);
                match task  {
                    Some(task) => println!("\nTarea {} ya existe\n", task.name),    
                    None => {let mut description = String::new();
                        println!("Ingrese la descripcion de la Tarea: ");
                        io::stdin().read_line(&mut description).unwrap();

                        Task::add_task(&mut tasks, name, description);
                    },
                }
            },
            "3" => {
                let mut name = String::new();
                println!("\nIngrese el nombre de la Tarea: ");
                io::stdin().read_line(&mut name).unwrap();
                Task::completed_task(&mut tasks, name.trim().to_string());
            },
            _ => break, 
        }
    }
}
