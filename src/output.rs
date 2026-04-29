use colored::Colorize;

pub enum OutputFormat {
    Pretty,
    Json,
    Plain,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "plain" => OutputFormat::Plain,
            _ => OutputFormat::Pretty,
        }
    }
}

pub struct OutputWriter {
    pub format: OutputFormat,
}

impl OutputWriter {
    pub fn new(format: OutputFormat) -> Self {
        OutputWriter { format }
    }

    pub fn print_header(&self, title: &str) {
        match self.format {
            OutputFormat::Pretty => {
                println!("{}", title.bold().underline().cyan());
            }
            OutputFormat::Plain => println!("{}", title),
            OutputFormat::Json => {}
        }
    }

    pub fn print_key_value(&self, key: &str, value: &str) {
        match self.format {
            OutputFormat::Pretty => {
                println!("  {} {}", key.green().bold(), value.white());
            }
            OutputFormat::Plain => println!("  {}: {}", key, value),
            OutputFormat::Json => {}
        }
    }

    pub fn print_added(&self, key: &str, value: &str) {
        match self.format {
            OutputFormat::Pretty => println!("  {} {}: {}", "+".green().bold(), key.green(), value.green()),
            OutputFormat::Plain => println!("  + {}: {}", key, value),
            OutputFormat::Json => {}
        }
    }

    pub fn print_removed(&self, key: &str, value: &str) {
        match self.format {
            OutputFormat::Pretty => println!("  {} {}: {}", "-".red().bold(), key.red(), value.red()),
            OutputFormat::Plain => println!("  - {}: {}", key, value),
            OutputFormat::Json => {}
        }
    }

    pub fn print_changed(&self, key: &str, old: &str, new: &str) {
        match self.format {
            OutputFormat::Pretty => {
                println!("  {} {}: {} {} {}", "~".yellow().bold(), key.yellow(), old.red().strikethrough(), "->".white(), new.green());
            }
            OutputFormat::Plain => println!("  ~ {}: {} -> {}", key, old, new),
            OutputFormat::Json => {}
        }
    }

    pub fn print_error(&self, msg: &str) {
        eprintln!("{} {}", "error:".red().bold(), msg);
    }

    pub fn print_info(&self, msg: &str) {
        match self.format {
            OutputFormat::Pretty => println!("{} {}", "info:".blue().bold(), msg),
            _ => println!("info: {}", msg),
        }
    }
}
