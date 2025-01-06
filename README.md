# AllanToolsRust

## Overview
AllanToolsRust is a Rust-based tool designed for calculating and analyzing Allan Deviation, a widely used metric for frequency stability analysis. The project provides functionality to compute Allan Deviation and related metrics with high performance and safety, leveraging the capabilities of the Rust programming language.

## Features
- Compute Allan Deviation and Overlapping Allan Deviation.
- Command-line interface for seamless integration into automated workflows.
- High performance and safety guarantees inherent to Rust.
- Modular and extensible design for future enhancements.

## Project Structure
```plaintext
.
├── Cargo.toml             # Project configuration file for Rust
├── src
│   ├── main.rs            # Main implementation in Rust
├── HP_8663A_16_MHz.tic    # Example of data
├── run.sh                 # Example of usage
├── output.adev            # Overlapped Allan Deviation for provided example
├── allan_plot.py          # Python script for plotting Allan Deviation results
```

## Getting Started

### Prerequisites
- Rust programming language (Rustup recommended).
- Cargo package manager (comes with Rust).

### Build Instructions
1. Clone the repository:
   ```bash
   git clone https://github.com/peterkaczorowski/AllanToolsRust.git
   cd AllanToolsRust
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the tool:
   ```bash
   cargo run --release -- <input_file> <sample_period> <data_type>
   ```

### Example Usage
Run Allan Deviation calculation using a test dataset:
```bash
cargo run --release -- HP_8663A_16_MHz.tic 0.004735426008968611 phase
```

Visualize results (example visualization not included in Rust but compatible with external tools like Python or gnuplot).

## Author
Piotr Kaczorowski

## License
This project is licensed under the [MIT License](LICENSE).

## Acknowledgments
This project draws inspiration from the AllanTools project for Python by Anders Wallin, available on GitHub at: https://github.com/aewallin/allantools. AllanToolsRust aims to provide similar functionality with the added benefits of Rust's performance and safety.


