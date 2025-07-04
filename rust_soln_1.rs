// Function for the right-hand side of the ODE: y' = cos(t) - y
/// The way to create a function in RUST is by using the 'fn' keyword,
/// also notice that the function takes two floating point 64 bit values as
/// input and returns a floating point value only.

fn f(t: f64, y: f64) -> f64 {
    t.cos() - y
}

// Euler's method function for any given n, notice that here we are taking 
/// and un sized integer as input and return two vectors as output, also 
/// notice that we don't use any keyword for returning.
fn euler_method(n: usize) -> (Vec<f64>, Vec<f64>) {
    let t_initial = 0.0; // let keyword is for initializing a variable.
    let t_final = 5.0;
    let y_initial = 1.0;
    let h = (t_final - t_initial) / n as f64;

    // Time points
    let mut t = Vec::with_capacity(n + 1); // mut keyword is very important 
    // as the mut keyword makes sure that we have the vector as a mutable one.
    // By default, it is unmutable, so we used mut here.
    for i in 0..=n {
        t.push(t_initial + i as f64 * h);
    }

    // Array to store y values
    let mut y = vec![0.0; n + 1];
    y[0] = y_initial;

    // Euler update
    for i in 0..n {
        y[i + 1] = y[i] + f(t[i], y[i]) * h;
    }

    (t, y) // Returning the two vectors y is the output using Euler's method.
}

fn main() {
    let n = 20; // Try 100 or 1000 as well
    let (t, y) = euler_method(n);

    println!("t\t\ty");
    for i in 0..=n {
        println!("{:.4}\t{:.6}", t[i], y[i]);
    }
}
