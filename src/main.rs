use clap::{Parser, Subcommand};

/// Absolem CLI
#[derive(Parser, Debug)]
#[command(name = "absolem")]
#[command(version, about = "A command-line tool for managing shell scripts.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute a registered shell script
    Call {
        /// Path to the registered script
        #[arg(short, long)]
        path: Option<String>,

        /// Arguments for the script
        #[arg(last = true)]
        script_args: Vec<String>,
    },

    /// Register an existing shell script
    Register {
        /// Path to the shell script file
        #[arg()]
        script_path: String,

        /// Path to register the script
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Create a new shell script
    Create {
        /// Path to register the script
        #[arg(short, long)]
        path: Option<String>,

        /// Open the script in an editor
        #[arg(long)]
        open: bool,
    },

    /// Record commands to a shell script
    Record {
        /// Disable replacing absolute paths with relative paths
        #[arg(long)]
        disable_replace_absolute_path: bool,
    },

    /// Rename a group or shell script
    Move {
        /// Current path of the script
        #[arg()]
        before_path: String,

        /// New path of the script
        #[arg()]
        after_path: String,
    },

    /// Delete a group or shell script
    Delete {
        /// Path to the script to delete
        #[arg()]
        path: String,
    },

    /// Print the content of a registered shell script
    Print {
        /// Path to the script to print
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Print the list of registered shell scripts
    List,

    /// Configuration commands
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigCommand {
    /// View the current configuration
    View,

    /// Set a configuration value
    Set {
        /// Configuration key
        #[arg()]
        key: String,

        /// Configuration value
        #[arg()]
        value: String,
    },

    /// Delete a configuration value
    Delete {
        /// Configuration key
        #[arg()]
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Call { path: register_path, script_args } => mock_call(register_path, script_args),
        Commands::Register { script_path, path } => mock_register(script_path, path),
        Commands::Create { path: register_path, open } => mock_create(register_path, open),
        Commands::Record { disable_replace_absolute_path } => mock_record(disable_replace_absolute_path),
        Commands::Move { before_path: before_register_path, after_path: after_register_path } => mock_move(before_register_path, after_register_path),
        Commands::Delete { path: register_path } => mock_delete(register_path),
        Commands::Print { path: register_path } => mock_print(register_path),
        Commands::List => mock_list(),
        Commands::Config { command } => match command {
            ConfigCommand::View => mock_config_view(),
            ConfigCommand::Set { key, value } => mock_config_set(key, value),
            ConfigCommand::Delete { key } => mock_config_delete(key),
        },
    }
}

fn mock_call(path: Option<String>, script_args: Vec<String>) {
    println!("Mock call: {:?}, args: {:?}", path, script_args);
}

fn mock_register(script_path: String, path: Option<String>) {
    println!("Mock register: {}, path: {:?}", script_path, path);
}

fn mock_create(path: Option<String>, open: bool) {
    println!("Mock create: {:?}, open: {}", path, open);
}

fn mock_record(disable_replace_absolute_path: bool) {
    println!("Mock record: disable_replace_absolute_path: {}", disable_replace_absolute_path);
}

fn mock_move(before_path: String, after_path: String) {
    println!("Mock move: {} -> {}", before_path, after_path);
}

fn mock_delete(path: String) {
    println!("Mock delete: {}", path);
}

fn mock_print(path: Option<String>) {
    println!("Mock print: {:?}", path);
}

fn mock_list() {
    println!("Mock list");
}

fn mock_config_view() {
    println!("Mock config view");
}

fn mock_config_set(key: String, value: String) {
    println!("Mock config set: {} = {}", key, value);
}

fn mock_config_delete(key: String) {
    println!("Mock config delete: {}", key);
}
