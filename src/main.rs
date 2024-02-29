use clipboard::{ClipboardContext, ClipboardProvider};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Inicia un hilo para monitorear el portapapeles
    let mut store: Vec<String> = Vec::new();
    let clipboard_thread = std::thread::spawn(move || {
        // Guarda el contenido actual del portapapeles
        let mut clipboard = ClipboardContext::new().expect("No se pudo acceder al portapapeles");
        let mut last_clipboard_content = clipboard.get_contents().unwrap_or_default();
        
        loop {
            // Obtiene el contenido actual del portapapeles
            let current_clipboard_content = clipboard.get_contents().unwrap_or_default();
            // add to array
            store.push(current_clipboard_content.clone());
            // Comprueba si el contenido ha cambiado
            if current_clipboard_content != last_clipboard_content {
                println!("Contenido del portapapeles cambiado: {:?}", store);
                // Puedes realizar cualquier acción que desees con el nuevo contenido

                // Actualiza el último contenido conocido del portapapeles
                last_clipboard_content = current_clipboard_content;
            }

            // Espera un breve período antes de verificar nuevamente
            sleep(Duration::from_millis(500));
        }
    });

    // Puedes realizar otras tareas aquí mientras esperas que el usuario copie algo

    // Espera a que el hilo del portapapeles finalice
    clipboard_thread.join().expect("Error al esperar al hilo del portapapeles");
}
