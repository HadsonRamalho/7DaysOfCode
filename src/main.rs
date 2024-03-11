use std::io;
use std::io::{BufRead, Read};

#[derive(Clone, Default, Debug)]
struct item{
    nome:String
}

#[derive(Copy, Clone, Default, Debug)]
struct quantidade{
    qtd:u32
}

#[derive(Default)]
struct ListaDeCompra{
    qtds:[quantidade; 10], itens:[item; 10]
}

fn stou(y:String) -> u32{
    let y: u32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => return 0
    };
    return y
}

fn main() {
    let lista:ListaDeCompra = Default::default();//
    let mut nome_item:String = Default::default();
    let mut qtd_item:String = Default::default();
    let mut qtd_itens:usize = 0;
    println!(" | Insira o nome do item: ");
    io::stdin().read_line(&mut nome_item).expect(" > Erro ao ler o nome do item");
    println!(" | Insira a quantidade que deseja adicionar: ");
    io::stdin().read_line(&mut qtd_item).expect(" > Erro ao ler a quantidade do item");
    let qtd_item = stou(qtd_item);
    println!("OK");
}