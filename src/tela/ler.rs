use std::io;

pub fn ler_dados() -> String {
    let mut dados = String::new();
    io::stdin().read_line(&mut dados).expect("Falha ao ler os dados");
    dados.trim().to_string()
}

pub fn ler_dados_int() -> usize {
    let mut dados = String::new();
    io::stdin().read_line(&mut dados).expect("Falha ao ler os dados");
    dados.trim().parse().expect("Erro ao converter dados para inteiro")
}