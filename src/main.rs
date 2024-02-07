fn main() {
    // To Define the learning rate (alpha), initial weights (w) and bias (b)
    let alpha = 1e-02;
    let mut w = 100.0;
    let mut b = 100.0;

    //To Define the training set
    let training_set: Vec<(f64, f64)> = vec![
        (1.5, 150.0),
        (1.75, 175.0),
        (2.0, 200.0),
        (2.25, 225.0),
        (2.5, 250.0),
        (2.75, 275.0)
    ];

    // Perform gradient descent to optimize parameters w and b
    (b, w) = gradient_descent(&training_set, alpha, ITER_NUM, b, w);

    // Print the optimized parameters
    println!("w: {}, b: {}", w, b);

    //  To Calculate and print the mean squared error
    println!("{:.1}", univariate_linear_regression(&training_set, b, w));
}

fn gradient_descent(training_set: &Vec<(f64, f64)>, alpha: f64, iter_num: u32, mut b: f64, mut w: f64) -> (f64, f64) {
    let mut cost;

    // Loop through the specified number of iterations
    for _ in 0..iter_num {
        // To Calculate new parameters using gradient descent
        let (new_b, new_w) = calculate_params(alpha, b, w);
        // To Calculate cost (mean squared error)
        cost = calculate_mean_squared_error_cost(new_b, new_w, training_set);
        
        // To Check if cost is 0, if so, break the loop
        if cost == 0.0 {
            break;
        }

        b = new_b;
        w = new_w;
    }
    (b, w)
}

fn calculate_params(alpha: f64, b: f64, w: f64) -> (f64, f64) {
    let new_w = w - alpha * sum_errors(b, w, 'w');
    let new_b = b - alpha * sum_errors(b, w, 'b');

    (new_b, new_w)
}

fn sum_errors(b: f64, w: f64, param: char) -> f64 {
    let mut sum = 0.0;
    // Loop through training set
    for (x, y) in Training_SET.iter() {
        if param == 'b' {
            // Calculate error for bias
            sum += univariate_linear_regression(*x, b, w) - y;
        } else {
            // Calculate error for weight
            sum += (univariate_linear_regression(*x, b, w) - y) * x;
        }
    }
    sum
}

fn univariate_linear_regression(x: f64, b: f64, w: f64) -> f64 {
    w * x + b
}

fn calculate_mean_squared_error_cost(b: f64, w: f64, training_set: &Vec<(f64, f64)>) -> f64 {
    let m = training_set.len() as f64;
    sum_squared_error(b, w, training_set) / m
}

fn sum_squared_error(b: f64, w: f64, training_set: &Vec<(f64, f64)>) -> f64 {
    let mut sum = 0.0;
    // Loop through training set
    for (x, y) in training_set.iter() {
        // Calculate squared error for each data point
        sum += (univariate_linear_regression(*x, b, w) - y).powi(2);
    }
    sum
}
