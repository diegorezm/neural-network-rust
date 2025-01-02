struct Neuron {
    input: f32,
    output: f32,
    weight: f32,
}

fn main() {
    let input = 20.0;
    let desired_out = 120.0;
    let mut c = 2.0;
    loop {
        let output = input * c;
        let error = desired_out - output;
        if output != 0.0 {
            eprint!("ERROR: {error}\n");
            if error < 0.0 {
                c += 0.5;
            } else {
                c -= 0.5;
            }
        } else {
            println!("OK");
            break;
        }
    }
}
