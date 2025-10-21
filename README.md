# LLM Engine

A Rust command-line application for running large language models locally using llama.cpp. Using .gguf files for llama-cpp compatibility (can be pulled from HuggingFace, most models have .gguf variants).

## Requirements

- Rust toolchain
- LLVM (for compilation)
- Windows with Visual Studio Build Tools

## Setup

1. Clone this repository
2. Place your model file in the `models/` directory (from HF or Ollama's remote repository)
3. Place your tokenizer file in the `models/` directory (preferrably from HF)

## Usage

Run the application with a prompt:

```bash
cargo run --release -- --prompt "Your question here"
```

## Model Files

My curret application uses:
- Model file: `models/mistral-7b-instruct-v0.2.Q4_K_M.gguf`
- Tokenizer file: `models/tokenizer.json`

You can modify the file paths in `src/main.rs` if needed.

## Building

The project includes environment configuration in `.cargo/config.toml` that automatically sets up the required build environment. 
```rust
[env]
LIBCLANG_PATH = "path/to/LLVM/bin"
NM_PATH = "path/to/LLVM/bin/llvm-nm.exe"
OBJCOPY_PATH = "path/to/LLVM/bin/llvm-objcopy.exe"
```

Simply run:

```bash
cargo build --release
```

## Dependencies

- llama_cpp: Interface to llama.cpp for model inference
- tokenizers: Text tokenization library
- clap: Command-line argument parsing
- anyhow: Error handling