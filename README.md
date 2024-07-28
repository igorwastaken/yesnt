# Yesnt Runtime
Just another JavaScript V8 runtime.

## Context of this Project
I was bored.

## Overview
Yesnt Runtime is a simple JavaScript runtime built using Rust and the V8 JavaScript engine. This project demonstrates how to integrate V8 with Rust and allows running JavaScript files from the command line.

## Features
- Execute JavaScript files using the V8 engine.
- Custom `printout` function to mimic `console.log` for printing messages from JavaScript to the console.
- Basic Read-Eval-Print Loop (REPL) support.

## Requirements
- Rust (with Cargo)
- V8 JavaScript engine

## Installation

### Install Rust
If you haven't installed Rust, you can install it using Rustup:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Setup V8
You need to have the V8 JavaScript engine available for linking. This can vary based on your platform and setup.

## Build
To build the Yesnt Runtime, clone the repository and run the following commands:

```sh
git clone https://github.com/igorwastaken/yesnt
cd yesnt
cargo build --release
```

## Usage
You can run JavaScript files using the following command:

```sh
./target/release/yesnt run path/to/your/script.js
```

### Example JavaScript File
Create a file named `test.js` with the following content:

```javascript
// test.js
printout("Hello from the Yesnt Runtime!");
```

Run the script:

```sh
./target/release/yesnt run test.js
```

### Using the REPL
You can also start a REPL session:

```sh
./target/release/yesnt
```

## Project Structure
- `src/main.rs`: The main entry point of the application.
- `src/functions/printout.rs`: Contains the definition of the `printout` function.
- `src/functions/mod.rs`: Module file for `functions`.

## License
This project is licensed under the MIT License.

## Acknowledgements
- Thanks to the tutorial: [Create Your Own JavaScript Runtime](https://dev.to/otterlord/create-your-own-javascript-runtime-10a4) for inspiration and guidance.
```