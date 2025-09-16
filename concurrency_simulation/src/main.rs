use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() { 
    // Creamos el canal (tx = transmitter, rx = receiver)
    let (tx, rx) = mpsc::channel();

    // Lista de clientes por cada cajero
    let cashier1 = vec!["Ana", "Luis", "Carlos"];
    let cashier2 = vec!["Marta", "Pedro", "Sofia"];
    let cashier3 = vec!["Juan", "Lucia"];

    // Lanzamos los threads simulando cajeros
    let cashiers = vec![cashier1, cashier2, cashier3];
    for (id, customers) in cashiers.into_iter().enumerate(){
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for customer in customers {
                // Simulamos tiempo de atencion
                thread::sleep(Duration::from_millis(500));
                // Enviamos mensaje al hilo principal
                let message = format!("Cajero {} atendio a {}", id + 1, customer);
                tx_clone.send(message).unwrap();
            }
        });
    }

    drop(tx); // Cerramos el transmisor original (para que el bucle termine)
    // El hilo principal recibe eb orden de llegada
    for received in rx {
        println!("-> {}", received);
    }

    println!("Todos los clientes fueron atendidos");
}
