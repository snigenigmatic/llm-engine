# LLM Engine

A Rust command-line application for running large language models locally using llama.cpp.

## Requirements

- Rust toolchain
- LLVM (for compilation)
- Windows with Visual Studio Build Tools

## Setup

1. Clone this repository
2. Place your model file in the `models/` directory
3. Place your tokenizer file in the `models/` directory

## Usage

Run the application with a prompt:

```
cargo run --release -- --prompt "Your question here"
```

## Model Files

The application expects:
- Model file: `models/mistral-7b-instruct-v0.2.Q4_K_M.gguf`
- Tokenizer file: `models/tokenizer.json`

You can modify the file paths in `src/main.rs` if needed.

## Building

The project includes environment configuration in `.cargo/config.toml` that automatically sets up the required build environment. Simply run:

```
cargo build --release
```

## Dependencies

- llama_cpp: Interface to llama.cpp for model inference
- tokenizers: Text tokenization library
- clap: Command-line argument parsing
- anyhow: Error handling