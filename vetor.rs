fn main(){
    let matriz_preco : [[f64;2]; 3] =[
        [15.00, 12.50],
        [13.00, 7.50],
        [100.00, 97.00],
    ];

    let mut medias: [f64;3] = [0.0; 3];

    for i in 0..3{
        medias[i] = (matriz_preco[i][0] + matriz_preco[i][1])/2.0
    }

    for media in medias.iter(){
        print!("{:.2}\t",media);
    }
}