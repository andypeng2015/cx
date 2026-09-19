use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Reference {
    #[value(name = "command-reference")]
    Command,
    DecisionTree,
    SetupAndRecovery,
    UsageInventory,
    OutputExamples,
}

pub fn print_core() {
    print!("{}", include_str!("../skills/cx/SKILL.md"));
}

pub fn print_reference(reference: Option<Reference>) {
    match reference {
        Some(reference) => print!("{}", reference.content()),
        None => print_reference_list(),
    }
}

impl Reference {
    fn content(self) -> &'static str {
        match self {
            Self::Command => include_str!("../skills/cx/references/command-reference.md"),
            Self::DecisionTree => include_str!("../skills/cx/references/decision-tree.md"),
            Self::SetupAndRecovery => include_str!("../skills/cx/references/setup-and-recovery.md"),
            Self::UsageInventory => include_str!("../skills/cx/references/usage-inventory.md"),
            Self::OutputExamples => include_str!("../skills/cx/references/output-examples.md"),
        }
    }
}

fn print_reference_list() {
    println!("Available skill references:");
    println!("  command-reference  Commands, filters, pagination, and output formats");
    println!("  decision-tree      Which navigation path to choose");
    println!("  setup-and-recovery Installation, grammars, cache, and failure recovery");
    println!("  usage-inventory    Distinguish semantic references from other usages");
    println!("  output-examples    Real command-output samples");
    println!();
    println!("Run: cx skill references <name>");
}
