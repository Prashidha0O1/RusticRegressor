

# Univariate Linear Regression with Gradient Descent

This Rust program implements univariate linear regression using gradient descent to optimize the parameters (slope and intercept) of the regression line. Univariate linear regression is a method used to model the relationship between a single independent variable and a dependent variable.

## Usage

1. **Clone the Repository**: Clone this repository to your local machine.

    ```bash
    git clone https://github.com/your-username/linear_regression.git
    ```

2. **Navigate to the Project Directory**: Enter the project directory.

    ```bash
    cd linear_regression
    ```

3. **Build and Run**: Build and run the Rust program.

    ```bash
    cargo run
    ```

## Description

### `main.rs`

This file contains the main logic of the program. It defines the training set, initializes the parameters (bias and weight), performs gradient descent to optimize the parameters, and calculates the mean squared error.

### `gradient_descent` Function

The `gradient_descent` function implements the gradient descent algorithm to update the parameters iteratively until convergence.

### `calculate_params` Function

The `calculate_params` function calculates the updated parameters (bias and weight) based on the gradient descent update rule.

### `sum_errors` Function

The `sum_errors` function calculates the sum of errors for the gradient descent algorithm.

### `univariate_linear_regression` Function

The `univariate_linear_regression` function calculates the predicted value of the dependent variable based on the independent variable, bias, and weight.

### `calculate_mean_squared_error_cost` Function

The `calculate_mean_squared_error_cost` function calculates the mean squared error cost function.

### `sum_squared_error` Function

The `sum_squared_error` function calculates the sum of squared errors for evaluating the model's performance.

## Dependencies

This project does not have any external dependencies beyond the standard Rust libraries.

## Contributing

Contributions are welcome! If you have suggestions, bug reports, or feature requests, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

