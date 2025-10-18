/// Output formatting with colors and branding
use owo_colors::OwoColorize;

/// Branding prefix for all anvs output
const BRAND: &str = "anvs";
const TAGLINE: &str = "automatic node version switching";

/// Print branded header message
pub fn print_header() {
    println!("{} - {}", BRAND.bright_cyan().bold(), TAGLINE.dimmed());
}

/// Print success message with green checkmark
pub fn success(msg: &str) {
    println!("{} {}", format!("{BRAND}:").bright_cyan(), msg.green());
}

/// Print info message
pub fn info(msg: &str) {
    println!("{} {}", format!("{BRAND}:").bright_cyan(), msg);
}

/// Print warning message with yellow color
pub fn warning(msg: &str) {
    println!("{} {}", format!("{BRAND}:").bright_cyan(), msg.yellow());
}

/// Print error message with red color
pub fn error(msg: &str) {
    eprintln!("{} {}", format!("{BRAND}:").bright_cyan(), msg.red().bold());
}

/// Print version switch success
pub fn switched(version: &str, plugin: &str) {
    println!(
        "{} {} {}",
        format!("{BRAND}:").bright_cyan(),
        "✓".green().bold(),
        format!("Switched to Node.js {version} (via {plugin})").green()
    );
}

/// Print installing message
pub fn installing(version: &str, plugin: &str) {
    println!(
        "{} {}",
        format!("{BRAND}:").bright_cyan(),
        format!("Installing Node.js {version} using {plugin}...").cyan()
    );
}

/// Print version mismatch warning
pub fn version_mismatch(required: &str, current: Option<&str>) {
    println!();
    warning(&format!(
        "⚠ Version mismatch: required {}, current {}",
        required,
        current.unwrap_or("none")
    ));
    println!();
    info("This may cause compatibility issues.");
}

/// Print install prompt
pub fn install_prompt(version: &str, plugin: &str) -> String {
    format!(
        "{} Node.js {} is not installed. Install it using {}?",
        format!("{BRAND}:").bright_cyan(),
        version,
        plugin
    )
}
