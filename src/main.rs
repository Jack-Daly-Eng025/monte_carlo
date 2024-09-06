use rand::Rng;


fn esti_pi(n_samp: usize) -> f64{
    let mut rng = rand::thread_rng();
    let mut inner_circ = 0;

    for i in 0..n_samp{
        let x = rng.gen();
        let y = rng.gen();
        if x * x + y* y < = 1.0 {
            inner_circ +=1;
        }
    }

    4.0 * inner_circ as f64 / n_samp as f64
}

fn main() {
    println!("Hello, world!");
}


