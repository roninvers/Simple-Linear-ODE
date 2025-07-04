# Numerical Solution of a Linear ODE: Euler Method, Analytical Comparison, and Rust Implementation

## Overview

This project explores the numerical solution of the first-order ordinary differential equation (ODE)  
 y' = cos(t) - y, y(0) = 1 
using the Euler method and compares it to the analytical solution. The work is implemented in both Python and Rust, with a focus on error analysis and visualization.

## Workflow Summary

### 1. **Euler Method in Python**
- Implemented the Euler method to solve the ODE for a small number of steps (e.g., 20).
- Output: Numerical solution for y at discrete time points.

### 2. **Visualization in Python**
- Plotted the Euler solution for n = 20 steps using Matplotlib.
- Helped visualize the basic behavior of the numerical solution.

### 3. **Analytical Solution and Error Analysis**
- Added the analytical solution for the ODE.
- Compared the Euler and analytical solutions for n = 100 steps.
- Plotted both solutions and the error term ( y_euler - y_exact).

### 4. **Higher Resolution and Error Plot**
- Increased the step size to n = 1000 for finer accuracy.
- Plotted the error term for n = 1000 to observe the improvement in accuracy with smaller step size.

### 5. **Rust Implementation**
- Re-implemented the Euler method in Rust, inspired by a YouTube tutorial for beginners.
- Noted similarities with C++ (syntax, structure) which eased the learning curve.
- The most challenging part was generating a CSV file from Rust.

### 6. **Exporting Data from Rust**
- Upgraded the Rust code to output a CSV file containing columns: `t`, `y_euler`, `y_exact`, and `error`.
- This facilitated easy data analysis and visualization in Python.

### 7. **Final Visualization in Python**
- Imported the CSV file into Python.
- Plotted:
  - The error term over time.
  - Both the Euler and analytical solutions for direct comparison.
- These plots provided clear insights into the accuracy and behavior of the numerical method.

## Key Learnings

- **Euler’s method** is a simple yet powerful tool for numerically solving ODEs, but its accuracy depends heavily on the step size.
- **Error analysis** is crucial for understanding the limitations of numerical methods.
- **Rust** is well-suited for scientific computing and offers strong performance, with syntax similar to C++.
- **Interoperability** between Rust and Python (via CSV) allows leveraging the strengths of both languages for computation and visualization.

## How to Run

1. **Python:**  
   - Run the Python scripts to compute and plot the solutions and errors.
2. **Rust:**  
   - Compile and run the Rust code to generate the CSV file.
   - Use Python to read and visualize the CSV data.

## Visualization Examples

- **Error Plot:** Shows how the error decreases with smaller step sizes.
- **Solution Comparison Plot:** Demonstrates how closely Euler’s method approximates the analytical solution for different step sizes.

## Acknowledgements

- Rust basics and CSV output inspired by [YouTube Rust tutorials](https://www.youtube.com/results?search_query=rust+for+beginners).
- Analytical solution and plotting techniques based on standard numerical analysis and Python scientific libraries.

**This project demonstrates a complete workflow from numerical ODE solution to error analysis and cross-language data handling for scientific computing.**
