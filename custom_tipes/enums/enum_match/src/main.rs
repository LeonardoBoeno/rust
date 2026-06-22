#[derive(Debug)]
enum Pedido {
    Hamburguer { tamanho: u8, queijo: bool },
    Pizza { sabor: String, fatias: u8 },
    Bebida(String),
    Sobremesa,
}

fn main() {
    let pedidos = vec![
        Pedido::Hamburguer { tamanho: 12, queijo: true },
        Pedido::Pizza { sabor: "Calabresa".into(), fatias: 8 },
        Pedido::Bebida("Coca-Cola".into()),
        Pedido::Sobremesa,
        Pedido::Hamburguer { tamanho: 20, queijo: false },
        Pedido::Bebida("Suco de Laranja".into()),
    ];

    for pedido in pedidos {
        println!("Processando pedido: {:?}", pedido);

        match pedido {
            // Desestruturação + match literal
            Pedido::Hamburguer { tamanho: 12, queijo: true } =>
                println!("Hambúrguer médio com queijo!"),

            // Desestruturação com variáveis
            Pedido::Hamburguer { tamanho, queijo } =>
                println!("Hambúrguer tamanho {} (queijo: {})", tamanho, queijo),

            // Match com string + guard
            Pedido::Bebida(ref nome) if nome == "Coca-Cola" =>
                println!("Bebida gelada especial!"),

            // Match com guard (condição extra)
            Pedido::Pizza { fatias, .. } if fatias > 6 =>
                println!("Pizza grande! {} fatias", fatias),

            // Desestruturação ignorando campos
            Pedido::Pizza { sabor, .. } =>
                println!("Pizza sabor {}", sabor),

            // Match simples
            Pedido::Sobremesa =>
                println!("Uma sobremesa deliciosa!"),

            // Agora o match é exaustivo
            Pedido::Bebida(nome) =>
                println!("Bebida comum: {}", nome),
        }

        println!("-----------------------------");
    }
}
