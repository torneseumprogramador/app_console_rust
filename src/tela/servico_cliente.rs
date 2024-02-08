use crate::{models::cliente::Cliente, tela::{ler::ler_dados_int, operacoes_basicas::esperar}};
use super::{ler::ler_dados, operacoes_basicas::limpar_tela};

pub fn incluir_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();

    let mut cliente: Cliente = Cliente::default();

    cliente.id = clientes.len() + 1;

    digitar_dados_do_cliente(&mut cliente);

    clientes.push(cliente);

    limpar_tela();
    println!("Cliente cadastrado com sucesso!");
    esperar(1);
}

fn digitar_dados_do_cliente(cliente: &mut Cliente) {
    println!("Digite o nome");
    cliente.nome = ler_dados();

    println!("Digite o CPF");
    cliente.cpf = ler_dados();

    println!("Digite o endereço");
    cliente.endereco = ler_dados();
}

pub fn alterar_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    if nao_tem_clientes(clientes) {
        return;
    }

    let id = captura_id();
    if let Some((indice, cliente)) = buscar_cliente_por_id(clientes, id){
        println!("{}", "-".to_string().repeat(40));
        println!("Alterando o cliente");
        println!("{}", "-".to_string().repeat(40));
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
        digitar_dados_do_cliente(&mut clientes[indice]);
        limpar_tela();
        println!("Cliente alterado com sucesso!");
    } else {
        limpar_tela();
        println!("Cliente não encontrado!")
    }

    esperar(1);
}


pub fn excluir_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    if nao_tem_clientes(clientes) {
        return;
    }

    let id = captura_id();
    if let Some((indice, cliente)) = buscar_cliente_por_id(clientes, id){
        println!("{}", "-".to_string().repeat(40));
        println!("Confirma a exclusão do cliente abaixo?");
        println!("{}", "-".to_string().repeat(40));
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
        println!("s - Sim\nn - Não");
        let opcao = ler_dados();
        if opcao == "s" {
            clientes.remove(indice);

            limpar_tela();
            println!("Cliente excluído com sucesso!");
            esperar(1);
        }
    } else {
        limpar_tela();
        println!("Cliente não encontrado!");
        esperar(1);
    }
}

fn buscar_cliente_por_id(clientes: &Vec<Cliente>, id: usize) -> Option<(usize, &Cliente)> {
    clientes.iter().enumerate().find(|(_, cliente)| cliente.id == id)
}

fn captura_id() -> usize {
    limpar_tela();
    println!("Digite o id do cliente");
    ler_dados_int()
}


pub fn listar_clientes(clientes: &mut Vec<Cliente>) {
    limpar_tela();

    if nao_tem_clientes(clientes) {
        return;
    }

    println!("{}", "-".to_string().repeat(40));
    for cliente in clientes {
        mostrar_cliente(cliente);
        println!("{}", "-".to_string().repeat(40));
    }
    
    println!("Digite enter para continuar ...");
    ler_dados();

}

fn nao_tem_clientes(clientes: &[Cliente]) -> bool {
    if clientes.len() == 0 {
        println!("Não existe clientes cadastrados!");
        esperar(1);
        return true;
    }

    return false;
}

fn mostrar_cliente(cliente: &Cliente) {
    println!("\
        ID: {}\n\
        Nome: {}\n\
        CPF: {}\n\
        Endereço: {}
    ", cliente.id, cliente.nome, cliente.cpf, cliente.endereco)
}
