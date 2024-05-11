use std::io;

fn while_loopping(){
    let mut soma = 0;
    let mut valor_entrada = String::new();
    io::stdin().read_line(&mut valor_entrada).expect("Erro ao ler o valor_entrada");
    let mut valor_i32 = convert_to_int(&valor_entrada);
    while valor_i32 != 0 {
        let mut r = valor_i32 % 10;
        soma = soma + r;
        valor_i32 = valor_i32 / 10;
    }
    println!("O valor da soma dos digitos {}", soma);
}

fn convert_to_int(data_input: & String) -> i32{
    let x: i32 = data_input.trim().parse::<i32>().unwrap(); // convertendo uma string para numero inteiro
    //trim() = ele vai remover os espaços da string
    //parse::<i32>() =  vai converter para o inteiro
    //unwrap() = O método unwrap() é usado para retornar o valor contido em um Result se ele for um Ok, ou lançar uma exceção se ele for um Err.
    //  No caso da linha de código apresentada, o método parse() é usado para tentar converter o valor de data_input para um número inteiro, retornando um Result. 
    // O método unwrap() é chamado em seguida para retornar o valor convertido em caso de sucesso ou lançar uma exceção em caso de falha. 
    // O valor retornado é armazenado em x.ele verifica se o valor vai ser do tipo None e retorna o erro
    x // retorna  a variavel
}

fn convert() {
    let mut number01 = String::new();
    io::stdin().read_line(&mut number01).expect("Erro ao ler number01");  // Estamos recebendo o numero 1 e passando a mensagem de erro caso há
    let mut number02 = String::new();
    io::stdin().read_line(&mut number02).expect("Erro ao ler number02"); // Estamos recebendo o numero 2 e passando a mensagem de erro caso há
    
    // Chamando a função de conversão para inteiro e passando a condional
    if convert_to_int(&number01) > convert_to_int(&number02){
        println!("O numero {} é maior que {}", number01, number02)
    }
    else{
        println!("O numero {} é menor ou igual que {}", number01, number02)
    }
}

fn conditional() {
    let number1: i32 = 24;
    let number2: i32 = 42;
    if number1 > number2 {
        println!("{} > {}", number1, number2)
    }
    else{
        println!("{} <= {}", number1, number2)
    }
}

fn data_types_numbers() {
    // INTEIROS
    let _numint: i32 = 23; //Por padrão ele vem como i32 = 
    let _numint2: i64 = 20254; //conseguimos alterar o padrão do inteiro de i8 até i128 bits
    let _numint3: u64 = 2000; // Quando passamos o u = unassigned, ou seja, ele não permite numeros negativos
    // FLOAT
    let _numfloat32: f32 = 6.7; // Números booleanos com 32 bits
    let _numfloat64: f64 = 61561.799; // Números booleanos com 64 bits
    //BOOLEAN
    let _boolfalse: bool = false; // falso
    let _booltrue: bool = true; // falso
}

fn declare_variables() {
    let mut name: &str =  "Douglas";
    name = "Vitória";
    println!("Hello {}!", name);
}

fn main() {
    // declare_variables();
    // data_types_numbers();
    // conditional();
    // convert();
    while_loopping();
}
