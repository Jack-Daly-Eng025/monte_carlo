//A monte-carlo simulation. 
use rand::Rng;


fn esti_pi(n_samp: usize) -> f64{
    let mut rng = rand::thread_rng();
    let mut inner_circ = 0;

    for _ in 0..n_samp{
        let x : f64 = rng.gen();
        let y : f64 = rng.gen();
        if x * x + y* y <= 1.0 {
            inner_circ +=1;
        }
    }

    4.0 * inner_circ as f64 / n_samp as f64
}

fn main() {
    let n = 10000;

    let  pi_esti = esti_pi(n);
    println!("The estimated value of pi is : {}", pi_esti);
}


