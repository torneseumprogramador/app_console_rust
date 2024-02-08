mod tela;
mod models;

use models::cliente::Cliente;
use tela::menu as menu;

fn main() {
    let mut clientes:Vec<Cliente> =  Vec::new();
    menu::mostrar_menu(&mut clientes);
}
