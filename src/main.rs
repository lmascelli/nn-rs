use rand::prelude::*;

struct RandomGenerator {
    rng: ThreadRng,
}

impl RandomGenerator {
    fn new() -> Self {
        Self {
            rng: rand::rng(),
        }
    }

    fn rand_float(&mut self) -> f32 {
        self.rng.next_u32() as f32 / u32::MAX as f32
    }
}

fn main() {
    let train = vec![
        (0., 0.),
        (1., 2.),
        (2., 4.),
        (3., 6.),
    ];
    
    let mut rng = RandomGenerator::new();
    println!("{}", rng.rand_float());
}
