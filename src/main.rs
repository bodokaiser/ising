use clap;

mod model;

fn main() {
    let matches = clap::App::new("Ising 2D")
        .version("1.0")
        .author("Bodo Kaiser")
        .get_matches();


    let mut ising = model::Ising::new(0.1, 5, 5);

    for _ in 0..100 {
        ising.step();
    }

    println!("m = {}", ising.magnetisation());
}
