use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String,
}

fn fetch_gcp_instances() -> String {
    println!("{}", "🔍 Scanning Google Cloud Compute resources...".cyan());

    // Attempts live GCP query via gcloud CLI if available
    let output = Command::new("gcloud")
        .args(&["compute", "instances", "list", "--format=json"])
        .output();

    match output {
        Ok(out) if out.status.success() && !out.stdout.is_empty() => {
            println!("{}", "☁️  Connected to live GCP project!".green());
            String::from_utf8_lossy(&out.stdout).to_string()
        }
        _ => {
            println!("{}", "ℹ️  Using local instances snapshot (mock / offline mode)...".yellow());
            fs::read_to_string("instances.json").unwrap_or_else(|_| "[]".to_string())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "==============================================".dimmed());
    println!("{}", "  🦥 SlothOps: Zero-Effort GCP FinOps Auditor ".bold().green());
    println!("{}", "  Powered by local Gemma open weights via Ollama".dimmed());
    println!("{}", "==============================================\n".dimmed());

    let instances_data = fetch_gcp_instances();

    let prompt = format!(
        "You are SlothOps, a lazy developer's FinOps assistant. \
        Inspect this raw JSON of GCP instances:\n{}\n\n\
        Identify which instances appear to be idle or test instances running without need. \
        Provide a short 3-line assessment of estimated wasted cost and provide the EXACT \
        one-line 'gcloud compute instances stop ...' command to stop them. \
        Be concise, calm, and straight to the point.",
        instances_data
    );

    println!("{}", "🧠 Asking Gemma 2 to audit idle instances...".magenta());

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&OllamaRequest {
            model: "gemma2:2b".to_string(),
            prompt,
            stream: false,
        })
        .send()
        .await;

    match res {
        Ok(response) => {
            let body: OllamaResponse = response.json().await?;
            println!("\n{}", "📋 Gemma's Lazy Audit Verdict:".bold().yellow());
            println!("{}", "--------------------------------------------------".dimmed());
            println!("{}", body.response.trim());
            println!("{}", "--------------------------------------------------".dimmed());
            println!("\n{}", "✅ Run the suggested gcloud command above to stop burning cash.".green());
        }
        Err(_) => {
            eprintln!("{}", "❌ Could not reach Ollama at localhost:11434.".red());
            eprintln!("Make sure Ollama is running with: 'ollama run gemma:2b'");
        }
    }

    Ok(())
}