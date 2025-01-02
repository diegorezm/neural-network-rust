## PART 1

Here, I created a guessing machine. Its purpose is to take an input (in this example, `10`) 
and attempt to guess the desired output using the following formula:  
`output = input * c`  

If the guess is incorrect, the machine calculates the error based on the desired output using this formula:  
`error = desired_out - output`  

Here’s the code:

```rust
fn main() {
    let input = 10.0;
    let desired_out = 26.0;
    let mut c = 2.0;
    loop {
        let output = input * c;
        let error = desired_out - output;
        if output != 0.0 {
            eprintln!("ERROR: {error}");
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
```

This is a very simple way to demonstrate a neural network. 
The user provides an input, and the machine tries to guess the desired output based on the training data. 
(In this case, the "training data" is just a hardcoded value.)

