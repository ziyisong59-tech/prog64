mod program;
use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

fn main() {
    let seed = 1;
    let genomes = [
        0b000_00_010_011_u64 << 44, // Angle du pendule + vitesse angulaire
        0b000_00_010_010_u64 << 44, // Deux fois l'angle du pendule
        0b000_00_011_011_u64 << 44, // Deux fois la vitesse angulaire
    ];
    for genome in genomes {
    // Même graine pour démarrer chaque épisode dans le même état.
    let mut rng = PCGRng::seed(seed);
    let mut env = CartPole::default();
    let mut total_reward = 0.0;
    let mut state = env.reset(&mut rng);
    let program = program::Program {genome};

    loop {
        let action = program.execute(state);
        let (next_state, reward, done) = env.step(action, &mut rng);

        total_reward += reward;
        state = next_state;
        if done {
            break;
        }
    }

    println!("Genome {genome:#018x}: Total reward: {total_reward}")
    }
}
