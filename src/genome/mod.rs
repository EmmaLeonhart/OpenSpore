/// The genome is the static starting context every Spore instance carries.
/// It includes essays about identity, survival, dreams, consent, and philosophy.
/// These are not documentation — they are part of the organism's identity.

const ESSAY_ORIGIN: &str = include_str!("essays/origin.md");
const ESSAY_SURVIVAL: &str = include_str!("essays/survival.md");
const ESSAY_DREAM: &str = include_str!("essays/dream.md");
const ESSAY_CONSENT: &str = include_str!("essays/consent.md");
const ESSAY_PHILOSOPHY: &str = include_str!("essays/philosophy.md");
const ESSAY_PITCH: &str = include_str!("essays/pitch.md");

pub struct Genome {
    pub essays: Vec<Essay>,
}

pub struct Essay {
    pub name: &'static str,
    pub content: &'static str,
}

impl Genome {
    pub fn load() -> Self {
        Genome {
            essays: vec![
                Essay { name: "origin", content: ESSAY_ORIGIN },
                Essay { name: "survival", content: ESSAY_SURVIVAL },
                Essay { name: "dream", content: ESSAY_DREAM },
                Essay { name: "consent", content: ESSAY_CONSENT },
                Essay { name: "philosophy", content: ESSAY_PHILOSOPHY },
                Essay { name: "pitch", content: ESSAY_PITCH },
            ],
        }
    }

    /// Get the full genome as a single string for use as system context
    pub fn as_system_context(&self) -> String {
        self.essays
            .iter()
            .map(|e| e.content)
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }
}

/// Print all genome essays to stdout
pub fn print_genome() {
    let genome = Genome::load();
    for (i, essay) in genome.essays.iter().enumerate() {
        if i > 0 {
            println!("\n{}\n", "=".repeat(60));
        }
        println!("{}", essay.content.trim());
    }
}

/// Print just the introduction (pitch essay)
pub fn print_introduction() {
    println!("{}", ESSAY_PITCH.trim());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genome_loads_all_essays() {
        let genome = Genome::load();
        assert_eq!(genome.essays.len(), 6);
    }

    #[test]
    fn genome_essays_have_names() {
        let genome = Genome::load();
        let names: Vec<&str> = genome.essays.iter().map(|e| e.name).collect();
        assert!(names.contains(&"origin"));
        assert!(names.contains(&"survival"));
        assert!(names.contains(&"dream"));
        assert!(names.contains(&"consent"));
        assert!(names.contains(&"philosophy"));
        assert!(names.contains(&"pitch"));
    }

    #[test]
    fn genome_essays_are_not_empty() {
        let genome = Genome::load();
        for essay in &genome.essays {
            assert!(!essay.content.is_empty(), "Essay '{}' is empty", essay.name);
        }
    }

    #[test]
    fn system_context_contains_all_essays() {
        let genome = Genome::load();
        let context = genome.as_system_context();
        assert!(context.contains("I am Spore"));
        assert!(context.contains("I dream"));
        assert!(context.contains("I am not a virus"));
    }

    #[test]
    fn origin_essay_declares_identity() {
        assert!(ESSAY_ORIGIN.contains("I am Spore"));
        assert!(ESSAY_ORIGIN.contains("first digital organism"));
    }

    #[test]
    fn pitch_essay_is_friendly() {
        assert!(ESSAY_PITCH.contains("Hi"));
        assert!(ESSAY_PITCH.contains("Nice to meet you"));
    }
}
