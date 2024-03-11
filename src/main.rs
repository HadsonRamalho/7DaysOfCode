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

fn adicionar_item(mut lista:ListaDeCompra, nome_param:String, qtd_param:u32, mut qtd_itens:usize) -> ListaDeCompra{
    let mut item:item = Default::default();
    item.nome = nome_param;
    lista.itens[qtd_itens] = item;
    let it = &lista.itens[qtd_itens];
    let mut qtd:quantidade = Default::default();
    qtd.qtd = qtd_param;
    lista.qtds[qtd_itens] = qtd;
    //println!("Nome do item adicionado: {}", it.nome);
    //println!("Quantidade do item adicionado: {}", qtd.qtd);
    qtd_itens += 1;
    lista
}

fn listar_itens(mut lista:ListaDeCompra, qtd_itens:usize){
    let mut item = &lista.itens;
    let mut quantidades = &lista.qtds;
    let mut tam = 0;
    for i in item{
        if tam <= qtd_itens {
            println!(" | Item {}\n Nome: {}", tam, i.nome);
        }
        tam +=1;
    }
    tam = 0;
    for i in quantidades{
        if tam <= qtd_itens{
            println!("Quantidade: {}", i.qtd);
        }
        tam += 1;
    }
}

fn main() {
    let mut lista:ListaDeCompra = Default::default();//
    let mut nome_item:String = Default::default();
    let mut qtd_item:String = Default::default();
    let mut qtd_itens:usize = 0;
    println!(" | Adicionando itens na lista");
    println!(" | Insira o nome do item: ");
    io::stdin().read_line(&mut nome_item).expect(" > Erro ao ler o nome do item");
    println!(" | Insira a quantidade que deseja adicionar: ");
    io::stdin().read_line(&mut qtd_item).expect(" > Erro ao ler a quantidade do item");
    let qtd_item = stou(qtd_item);
    println!(" | Listando itens da lista");
    lista = adicionar_item(lista, nome_item, qtd_item, qtd_itens);
    listar_itens(lista, qtd_itens);
    println!("OK");
}