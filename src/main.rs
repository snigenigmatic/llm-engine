use anyhow::Result;
use llama_cpp::{LlamaModel, LlamaParams, SessionParams};
use llama_cpp::standard_sampler::StandardSampler;
use tokenizers::Tokenizer;
use std::path::PathBuf;
use std::io::{self, Write};
use::clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // Path to model file
    #[arg(short, long, default_value = "models/mistral-7b-instruct-v0.2.Q4_K_M.gguf")]
    model: PathBuf, 

    // Path to tokenizer.json file
    #[arg(short, long, default_value = "models/tokenizer.json")]
    tokenizer: PathBuf,

    // Prompt to send to the model
    #[arg(short, long)]
    prompt: Option<String>,

    // Max tokens
    #[arg(short = 'm', long, default_value_t = 128)]
    max_tokens: usize,

    // Temperature
    #[arg(short = 't', long, default_value_t = 0.7)]
    temperature: f32,

    // Top-k
    #[arg(short = 'k', long, default_value_t = 40)]
    top_k: usize,

    // Top-p
    #[arg(short = 'p', long, default_value_t = 0.95)]
    top_p: f32,

}

fn main() -> Result<()> {
    let args = Args::parse();

    let prompt = if let Some(p) = args.prompt {
        p
    } else {
        print!("Enter your prompt: ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        buf.trim().to_string()
    };

    println!("\n[Loading model: {}]", args.model.display());
    let model = LlamaModel::load_from_file(args.model, LlamaParams::default())?;

    println!("[Loading tokenizer: {}]", args.tokenizer.display());
    let tokenizer = Tokenizer::from_file(args.tokenizer).map_err(|e| anyhow::anyhow!(e))?;

    println!("\n[Prompt]: {}\n", prompt);
    print!("[Response]: ");
    io::stdout().flush()?;

    // Create a session and run a simple completion using the standard sampler.
    let mut session = model.create_session(SessionParams::default())?;
    session.advance_context(&prompt)?;

    let mut decoded_tokens = 0usize;
    let mut completions = session.start_completing_with(StandardSampler::default(), args.max_tokens)?.into_strings();
    for completion in completions {
        print!("{}", completion);
        io::stdout().flush()?;

        decoded_tokens += 1;
        if decoded_tokens >= args.max_tokens {
            break;
        }
    }

    println!("\n\n[Done]");
    Ok(())
}