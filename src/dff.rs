// Initial state for clock.
struct DFF {
    state: u64,
}

impl DFF {
    // Define default state in tick that can be updated.
    fn tick() -> Self {
        Self { state: 0 }
    }

    // Update state in clock with input and return as output.
    fn clock_cycle(&mut self, input: u64) -> u64 {
        let output = self.state;
        self.state = input;
        output
    }
}

// Receive inputs, with an unassigned range.
fn data<const N: usize>(inputs: [u64; N]) -> u64 {
    // Create flip flop with initial state 0.
    let mut dff = DFF::tick();
    let mut outp = 0;
    let mut i = 0;
    
    while i < inputs.len() {
        // Update every end of cycle (the last cycle remains in cycle, so
        // it isn't given as unit).
        outp = dff.clock_cycle(inputs[i]);
        println!(
            "Cycle {}: Input = {}: Output = {}: Stored = {}",
            i + 1,
            inputs[i],
            dff.state,
            outp,
        );
        i += 1;
    }

    outp
}

fn main() {
    let data_given = [1, 2, 3, 4, 5, 6, 7];
    let test = data(data_given);
    println!("{}", test);
}