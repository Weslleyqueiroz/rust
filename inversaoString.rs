use std::io;

fn main(){
    let mut _escreva = String::new();
    println!("Digite uma palavra : ");

    io::stdin()
    .read_line(&mut _escreva)
    .expect("Erro, digite novamente!");

    println!("Você digitou {_escreva}");

    reverse(&_escreva);
}

    fn reverse(input: &str){
        let reversed: String = input.chars() .rev() .collect();
        println!("A palavra invertida é : {} ",reversed);
    }
    