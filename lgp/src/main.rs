mod program;
use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

fn main() {
    let seed = 1;
    let mut rng = PCGRng::seed(seed);
    let mut env = CartPole::default();
    let mut total_reward = 0.0;
    let mut state = env.reset(&mut rng);
    let program = program::Program {
        genome: 0b000_00_010_011_u64 << 44,
    };

    loop {
        let action = program.execute(state);
        let (next_state, reward, done) = env.step(action, &mut rng);

        total_reward += reward;
        state = next_state;
        if done {
            break;
        }
    }

    println!("Total reward: {}", total_reward)
}
