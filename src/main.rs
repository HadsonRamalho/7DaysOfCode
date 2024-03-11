use std::io;

#[derive(Clone, Default, Debug)]
struct Item{
    nome:String
}

#[derive(Copy, Clone, Default, Debug)]
struct Quantidade{
    qtd:u32
}

#[derive(Default)]
struct ListaDeCompra{
    qtds:[Quantidade; 10], itens:[Item; 10]
}

fn stou(y:String) -> u32{
    let y: u32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => return 0
    };
    return y
}

fn adicionar_item(mut lista:ListaDeCompra, nome_param:String, qtd_param:u32,mut qtd_itens:usize) -> (ListaDeCompra, u32){
    let mut item:Item = Default::default();
    item.nome = nome_param;
    lista.itens[qtd_itens] = item;
    let mut qtd:Quantidade = Default::default();
    qtd.qtd = qtd_param;
    lista.qtds[qtd_itens] = qtd;
    let num= qtd_itens.to_string();
    let num = stou(num);
    let t: (ListaDeCompra, u32) = (lista, num);
    t
}

fn listar_itens(lista:ListaDeCompra, mut qtd_itens: u32) -> ListaDeCompra{
    let item = &lista.itens;
    let quantidades = &lista.qtds;
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
    qtd_itens+=1;
    lista
}

fn main() {
    let mut lista:ListaDeCompra = Default::default();//
    let mut nome_item:String = Default::default();
    let mut qtd_item:String = Default::default();
    let mut qtd_itens:u32 = 0;
    println!(" | Adicionando itens na lista");
    println!(" | Insira o nome do item: ");
    io::stdin().read_line(&mut nome_item).expect(" > Erro ao ler o nome do item");
    println!(" | Insira a quantidade que deseja adicionar: ");
    io::stdin().read_line(&mut qtd_item).expect(" > Erro ao ler a quantidade do item");
    let qtd_item = stou(qtd_item);
    println!(" | Listando itens da lista");
    let tupla_lista_qtd_nome:(ListaDeCompra, u32);
    tupla_lista_qtd_nome =
        adicionar_item(lista, nome_item, qtd_item, qtd_itens as usize);
    lista =
        tupla_lista_qtd_nome.0;
    qtd_itens =
        tupla_lista_qtd_nome.1 as usize as u32;
    lista =
        listar_itens(lista, qtd_itens);

    println!("OK");
}