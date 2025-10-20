/// Output formatting with colors and branding
use owo_colors::OwoColorize;

/// Brand color (lime green)
pub const BRAND_COLOR: owo_colors::Rgb = owo_colors::Rgb(50, 205, 50);
/// Success color (green)
pub const SUCCESS_COLOR: owo_colors::Rgb = owo_colors::Rgb(34, 197, 94);
/// Warning color (yellow)
pub const WARNING_COLOR: owo_colors::Rgb = owo_colors::Rgb(251, 191, 36);
/// Error color (red)
pub const ERROR_COLOR: owo_colors::Rgb = owo_colors::Rgb(239, 68, 68);
/// Info color (blue)
pub const INFO_COLOR: owo_colors::Rgb = owo_colors::Rgb(59, 130, 246);

/// Branding prefix for all anvs output
const BRAND: &str = "⚡ Automatic Node Version Switcher";
const TAGLINE: &str = "automatic node version switching";

/// Print branded header message
pub fn print_header() {
    println!(
        "{} - {}",
        BRAND.truecolor(50, 205, 50).bold(),
        TAGLINE.dimmed()
    );
}

/// Print a brand message with custom text
pub fn brand(msg: &str) {
    println!("{}", msg.truecolor(50, 205, 50).bold());
}

/// Print success message with green checkmark
pub fn success(msg: &str) {
    println!("{}", format!("{BRAND}:").truecolor(50, 205, 50));
    println!("{}", msg.green());
}

/// Print info message
pub fn info(msg: &str) {
    println!("{}", format!("{BRAND}:").truecolor(50, 205, 50));
    println!("{msg}");
}

/// Print warning message with yellow color
pub fn warning(msg: &str) {
    println!("{}", format!("{BRAND}:").truecolor(50, 205, 50));
    println!("{}", msg.yellow());
}

/// Print error message with red color
pub fn error(msg: &str) {
    eprintln!("{}", format!("{BRAND}:").truecolor(50, 205, 50));
    eprintln!("{}", msg.red().bold());
}

/// Print version switch success
pub fn switched(version: &str, _plugin: &str) {
    println!("{}", format!("{BRAND}:").truecolor(50, 205, 50));
    println!(
        "{} {}",
        "✓".green().bold(),
        format!("Switching to {version}").truecolor(INFO_COLOR.0, INFO_COLOR.1, INFO_COLOR.2)
    );
}

/// Print installing message
pub fn installing(version: &str, plugin: &str) {
    println!("{}", format!("{BRAND}:").truecolor(50, 205, 50));
    println!(
        "{}",
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
        format!("{BRAND}:").truecolor(50, 205, 50),
        version,
        plugin
    )
}
