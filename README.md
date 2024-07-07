# 30 Days Lost in Space with Rust

This repository contains open-source solutions for the "Adventure Kit: 30 Days Lost in Space" course, implemented in Rust. The course is designed to teach the fundamentals of electronics and programming through daily challenges, and I've chosen to complete it using Rust for the embedded systems.

## Getting Started

These instructions will guide you through the setup and execution of the projects on a MacOS system.

### Prerequisites

Before starting, ensure you have the following installed:

- Xcode Command Line Tools
- Homebrew

Install the required tools using the following commands:

```bash
xcode-select --install # if you haven't already done so
brew tap osx-cross/avr
brew install avr-gcc avrdude
cargo +stable install ravedude
```

### Environment Configuration

Set the RAVEDUDE port to communicate with your Arduino Uno:

```bash
export RAVEDUDE_PORT=/dev/cu.usbserial-14440
```

### Running the Examples

Day 1: Incoming Broadcast from InventrCorp
```bash
cargo run --example day1
```

## Contributing

Feel free to fork this repository and submit pull requests. Contributions are welcome!

1. Fork the repo
2. Create a new branch (`git checkout -b feature-branch`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin feature-branch`)
5. Create a new Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [InventrKits](https://learn.inventr.io) for the original "30 Days Lost in Space" course.
- [Rahix](https://github.com/Rahix/avr-hal-template) for the AVR HAL template.
