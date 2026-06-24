fn main() {
    let pontos = vec![
        (0, 0, 0),
        (0, 5, -3),
        (10, -2, 3),
        (3, 8, 4),
        (7, 1, 2),
        (1, 2, 2),
        (5, 5, 5),
    ];

    for ponto in pontos {
        println!("Analisando ponto: {:?}", ponto);

        match ponto {
            // Padrão literal completo
            (0, 0, 0) =>
                println!("Origem absoluta do espaço!"),

            // Primeiro elemento fixo, resto variável
            (0, y, z) =>
                println!("Eixo X zerado → y={}, z={}", y, z),

            // Último elemento fixo
            (.., 2) =>
                println!("Z é 2, ignorando o resto"),

            // Primeiro e último fixos
            (3, .., 4) =>
                println!("Começa com 3 e termina com 4"),

            // Guard: soma positiva
            (x, y, z) if x + y + z > 10 =>
                println!("Soma dos elementos é maior que 10"),

            // Guard: valores negativos
            (x, y, z) if x < 0 || y < 0 || z < 0 =>
                println!("Algum valor é negativo"),

            // Padrão parcial com ignorância
            (x, ..) =>
                println!("Começa com {}, resto ignorado", x),
        }

        println!("-----------------------------");
    }
}
