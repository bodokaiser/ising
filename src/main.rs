use clap;
use clap::value_t;

mod model;

fn main() {
    let matches = clap::App::new("Ising 2D")
        .version("1.0")
        .author("Bodo Kaiser")
        .arg(
            clap::Arg::with_name("steps")
                .long("steps")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("beta")
                .long("beta")
                .default_value("1")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("size")
                .long("size")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("burn-in-steps")
                .long("burn-in-steps")
                .default_value("0")
                .takes_value(true),
        )
        .get_matches();

    let steps = value_t!(matches, "steps", u32).unwrap_or_else(|e| e.exit());
    let beta = value_t!(matches, "beta", f32).unwrap_or_else(|e| e.exit());
    let size = value_t!(matches, "size", u32).unwrap_or_else(|e| e.exit());
    let burn_in_steps = value_t!(matches, "burn-in-steps", u32).unwrap_or_else(|e| e.exit());

    let mut ising = model::Ising::new(beta, size, size);

    println!("step,energy,magnetisation,susceptibility");

    for step in 0..steps {
        ising.step();

        if step >= burn_in_steps {
            println!(
                "{},{},{},{}",
                step,
                ising.energy(),
                ising.magnetisation(),
                ising.susceptibility()
            );
        }
    }
}
