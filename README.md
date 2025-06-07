# 🧮 Rust CLI Calculator

A simple and interactive command-line calculator built with Rust. This project is great for beginners learning Rust fundamentals like error handling, input/output, pattern matching, and basic control flow.

---

## ✨ Features

- ✅ Supports addition (`+`), subtraction (`-`), multiplication (`*` or `x`), and division (`/`)
- 🚫 Handles invalid input (e.g., text instead of numbers, divide by zero)
- 🔁 Keeps prompting until valid input is received
- 🎓 Great as a first Rust project

---

## 📦 Installation

Make sure you have Rust installed. If not:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


```

## Clone the repo and run

```bash
git clone https://github.com/GolgothaAksanti/cli-rust-calculator.git
cd cli-rust-calculator
cargo run


```

## Example Usage

```bash
🧮 Rust CLI Calculator
Enter the first number: 12
Enter the second number: 4
Enter an operator (+, -, *, /): /
Result: 3

```

handles errors gracefully:

```Bash
Enter the first number: hello
❌ Invalid number. Please try again.

Enter the second number: -
❌ Invalid number. Please try again.

Enter an operator (+, -, *, /): $
❌ Invalid operator: $

```

## 🧠 What You'll Learn

- How to read from `stdin` and print to `stdout`

- Using `.trim()`, `.parse::<f64>()`, and handling `Result` types

- Writing clean and idiomatic Rust for command-line tools

## Project Structure

```css
cli-rust-calculator/
├── src/
│   └── main.rs     # Main calculator logic
├── Cargo.toml      # Project manifest

```

## 🤝 Contributing

Pull requests and suggestions are welcome! For major changes, please open an issue first to discuss what you’d like to change.

## 💡 Bonus Ideas

- Add support for exponentiation (`^`)

- Support chaining multiple operations

- Build a history of calculations

- Add a GUI with `egui` or `gtk-rs`
