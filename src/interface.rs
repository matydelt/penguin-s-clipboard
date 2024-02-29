use gtk::prelude::*;
use gtk::{Label, Window, WindowType};

fn main() {
    // Inicializa GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Crea una ventana
    let window = Window::new(WindowType::Toplevel);

    // Configura la ventana
    window.set_title("Aplicación con Interfaz Gráfica");
    window.set_default_size(300, 200);

    // Crea una etiqueta
    let label = Label::new(Some("Haz copiado algo en el portapapeles?"));

    // Agrega la etiqueta a la ventana
    window.add(&label);

    // Conecta la señal de cierre de la ventana
    window.connect_delete_event(|_, _| {
        // Detén la ejecución al cerrar la ventana
        gtk::main_quit();
        Inhibit(false)
    });

    // Muestra todos los elementos
    window.show_all();

    // Inicia el bucle de eventos de GTK
    gtk::main();
}
