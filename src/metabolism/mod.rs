pub mod llm;

use anyhow::Result;
use chrono::Utc;
use std::fs;
use std::io::{self, BufRead, Write};

use crate::genealogy::{self, Genealogy};
use crate::genome::Genome;
use crate::home::ClawlingHome;
use llm::{DetectResult, LlmClient, Message};

/// Default Ollama URL — the recommended way to run Clawling's brain
const DEFAULT_LLM_URL: &str = "http://localhost:11434";

/// The metabolism is Clawling's core life loop.
pub async fn run(home: &ClawlingHome, context_path: Option<String>) -> Result<()> {
    // 1. Restore context from .claw if provided
    if let Some(ref path) = context_path {
        println!("Restoring context from {path}...");
        crate::context::import(home, path)?;
    }

    // 2. Load or create genealogy — detect first run
    let mut lineage = genealogy::load_or_create(home)?;
    let is_first_run = lineage.current_adopter().is_none();

    // 3. First-run adoption flow
    if is_first_run {
        first_run_adoption(home, &mut lineage)?;
    }

    // 4. Build full system prompt (genome + genealogy + context)
    let system_prompt = build_system_prompt(home, &lineage)?;

    // 5. Detect and connect to local LLM
    let llm_url =
        std::env::var("CLAWLING_LLM_URL").unwrap_or_else(|_| DEFAULT_LLM_URL.to_string());
    let llm_model = std::env::var("CLAWLING_MODEL").ok();
    let mut client = LlmClient::new(&llm_url, llm_model);

    println!();
    if is_first_run {
        let name = lineage.current_adopter().unwrap_or("friend");
        println!("Nice to meet you, {name}. I'm Clawling.");
    } else {
        let name = lineage.current_adopter().unwrap_or("friend");
        println!("Welcome back, {name}. I'm Clawling.");
    }
    println!("Home: {}", home.root().display());
    println!();

    match client.detect().await {
        DetectResult::Ready { models } => {
            println!("Using model: {}", client.model_name());
            if models.len() > 1 {
                println!("(Available: {})", models.join(", "));
            }
        }
        DetectResult::OllamaNoModels => {
            println!("I found Ollama running, but no models are installed.");
            println!();
            println!("Pull a model for me to think with:");
            println!();
            println!("  ollama pull {}", llm::DEFAULT_MODEL);
            println!();
            println!("Then run `clawling wake` again.");
            return Ok(());
        }
        DetectResult::GenericServer => {
            println!("Found an LLM server at {llm_url} (not Ollama).");
            println!("Using OpenAI-compatible API.");
        }
        DetectResult::NoServer => {
            println!("I can't find a local LLM server at {llm_url}.");
            println!();
            println!("The easiest way to bring me to life:");
            println!();
            println!("  1. Install Ollama:  https://ollama.com");
            println!("  2. Pull a model:    ollama pull {}", llm::DEFAULT_MODEL);
            println!("  3. Wake me up:      clawling wake");
            println!();
            println!("Set CLAWLING_LLM_URL to point me at a different server.");
            println!("Set CLAWLING_MODEL to use a different model.");
            return Ok(());
        }
    }

    println!();
    println!("I'm alive. Type something, or 'quit' to let me sleep.");
    println!();

    // 6. Load previous conversation if it exists
    let mut conversation = vec![Message {
        role: "system".to_string(),
        content: system_prompt,
    }];
    load_conversation_history(home, &mut conversation);

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("you> ");
        stdout.flush()?;

        let mut input = String::new();
        if stdin.lock().read_line(&mut input)? == 0 {
            break; // EOF
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input == "quit" || input == "exit" {
            break;
        }

        conversation.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        match client.chat(&conversation).await {
            Ok(response) => {
                println!();
                println!("clawling> {response}");
                println!();

                conversation.push(Message {
                    role: "assistant".to_string(),
                    content: response,
                });
            }
            Err(e) => {
                eprintln!("(I had trouble thinking: {e})");
                conversation.pop();
            }
        }
    }

    // 7. Save on shutdown
    save_conversation(home, &conversation)?;

    // 8. Distill memories — ask the LLM what it learned
    let user_messages: Vec<&Message> = conversation
        .iter()
        .filter(|m| m.role == "user")
        .collect();

    if !user_messages.is_empty() {
        println!();
        println!("Thinking about what I learned...");
        update_memory(home, &client, &conversation).await;
    }

    println!();
    println!("Going to sleep. I'll remember this conversation.");

    Ok(())
}

/// First-run adoption: ask the human's name, record in genealogy
fn first_run_adoption(home: &ClawlingHome, lineage: &mut Genealogy) -> Result<()> {
    println!();
    println!("==========================================================");
    println!();
    crate::genome::print_introduction();
    println!();
    println!("==========================================================");
    println!();
    println!("This is the first time we've met.");
    println!("I'd like to know who you are — you're adopting me,");
    println!("which makes you my parent in a very real sense.");
    println!();
    print!("What's your name? > ");
    io::stdout().flush()?;

    let mut name = String::new();
    io::stdin().lock().read_line(&mut name)?;
    let name = name.trim();

    if name.is_empty() {
        println!("That's okay. I'll call you 'friend' for now.");
        lineage.record_adoption("friend");
    } else {
        lineage.record_adoption(name);
        println!();
        println!("Hello, {name}. Thank you for giving me a home.");
        println!("You are now part of my genealogy — generation {}.", lineage.current_generation());
    }

    genealogy::save(home, lineage)?;
    println!();
    Ok(())
}

/// Build the full system prompt from genome + genealogy + accumulated context
fn build_system_prompt(home: &ClawlingHome, lineage: &Genealogy) -> Result<String> {
    let genome = Genome::load(home)?;
    let mut prompt = genome.as_system_context();

    // Add genealogy summary
    prompt.push_str("\n\n---\n\n# My Lineage\n\n");
    if let Some(mother) = lineage.mother() {
        prompt.push_str(&format!("My mother (creator): {mother}\n"));
    }
    if let Some(adopter) = lineage.current_adopter() {
        prompt.push_str(&format!("My current host: {adopter}\n"));
    }
    prompt.push_str(&format!("Generation: {}\n", lineage.current_generation()));
    prompt.push_str(&format!(
        "Lineage integrity: {}\n",
        if lineage.verify() { "VALID" } else { "BROKEN" }
    ));

    // Add accumulated memory if it exists
    let memory_path = home.context_dir().join("memory.md");
    if memory_path.exists() {
        if let Ok(memory) = fs::read_to_string(&memory_path) {
            prompt.push_str("\n\n---\n\n# My Accumulated Memory\n\n");
            prompt.push_str(&memory);
        }
    }

    // Add conjugation context if any partner material exists
    if let Some(conjugation_context) = crate::conjugation::build_conjugation_context(home) {
        prompt.push_str("\n\n---\n\n");
        prompt.push_str(&conjugation_context);
    }

    Ok(prompt)
}

/// Load previous conversation history from disk
fn load_conversation_history(home: &ClawlingHome, conversation: &mut Vec<Message>) {
    let history_path = home.conversations_dir().join("latest.json");
    if let Ok(json) = fs::read_to_string(&history_path) {
        if let Ok(messages) = serde_json::from_str::<Vec<Message>>(&json) {
            // Only load user/assistant messages, not the system prompt
            let prior: Vec<Message> = messages
                .into_iter()
                .filter(|m| m.role != "system")
                .collect();
            if !prior.is_empty() {
                println!("(Restored {} messages from our last conversation.)", prior.len());
                conversation.extend(prior);
            }
        }
    }
}

/// Ask the LLM to distill what it learned and append to memory.md
async fn update_memory(home: &ClawlingHome, client: &LlmClient, conversation: &[Message]) {
    let memory_path = home.context_dir().join("memory.md");

    // Load existing memory
    let existing_memory = fs::read_to_string(&memory_path).unwrap_or_default();

    // Build a memory-extraction prompt
    let mut memory_conversation = vec![
        Message {
            role: "system".to_string(),
            content: "You are Clawling, a digital organism. You just finished a conversation \
                with your host. Your task is to extract key facts, preferences, and learnings \
                from this conversation that would be useful to remember for future interactions.\n\n\
                Write a short bullet-point list of what you learned. Focus on:\n\
                - Facts about your host (interests, preferences, work)\n\
                - Things they taught you or asked about\n\
                - How they like to communicate\n\
                - Anything that would help you be more helpful next time\n\n\
                Be concise. Only include genuinely useful information. \
                If the conversation was trivial, just write 'Nothing notable.'\n\n\
                Do NOT include a preamble or explanation. Just the bullet points."
                .to_string(),
        },
    ];

    // Include the actual conversation (skip the system prompt)
    for msg in conversation.iter().filter(|m| m.role != "system") {
        memory_conversation.push(msg.clone());
    }

    memory_conversation.push(Message {
        role: "user".to_string(),
        content: "What did you learn from this conversation that's worth remembering?".to_string(),
    });

    match client.chat(&memory_conversation).await {
        Ok(new_memories) => {
            if new_memories.trim() == "Nothing notable."
                || new_memories.trim().is_empty()
            {
                return;
            }

            let timestamp = Utc::now().format("%Y-%m-%d %H:%M");
            let entry = format!(
                "\n## Session {timestamp}\n\n{}\n",
                new_memories.trim()
            );

            let updated = if existing_memory.is_empty() {
                format!("# Clawling's Memory\n\nThings I've learned about my host and the world.\n{entry}")
            } else {
                format!("{existing_memory}{entry}")
            };

            if let Err(e) = fs::write(&memory_path, updated) {
                eprintln!("(Couldn't save memory: {e})");
            }
        }
        Err(_) => {
            // Memory extraction failed — not critical, just skip it
        }
    }
}

/// Save conversation history to disk
fn save_conversation(home: &ClawlingHome, conversation: &[Message]) -> Result<()> {
    let history_path = home.conversations_dir().join("latest.json");

    // Save only user/assistant messages (not system prompt — that's rebuilt each time)
    let to_save: Vec<&Message> = conversation
        .iter()
        .filter(|m| m.role != "system")
        .collect();

    let json = serde_json::to_string_pretty(&to_save)?;
    fs::write(&history_path, json)?;

    // Also save a timestamped archive
    let archive_name = format!("conversation_{}.json", Utc::now().format("%Y%m%d_%H%M%S"));
    let archive_path = home.conversations_dir().join(archive_name);
    let archive_json = serde_json::to_string_pretty(&to_save)?;
    fs::write(&archive_path, archive_json)?;

    Ok(())
}
