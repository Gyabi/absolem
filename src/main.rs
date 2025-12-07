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

    let absolem_core = absolem_core::AbsolemCore::new();

    match cli.command {
        Commands::Call {
            path: register_path,
            script_args,
        } => absolem_core.call_shell_script(register_path, script_args),
        Commands::Register { script_path, path } => {
            absolem_core.register_shell_script(script_path, path)
        }
        Commands::Create {
            path: register_path,
            open,
        } => absolem_core.create_shell_script(register_path, open),
        Commands::Record {
            disable_replace_absolute_path,
        } => absolem_core.record_shell_script(disable_replace_absolute_path),
        Commands::Move {
            before_path: before_register_path,
            after_path: after_register_path,
        } => absolem_core.move_shell_script(before_register_path, after_register_path),
        Commands::Delete {
            path: register_path,
        } => absolem_core.delete_shell_script(register_path),
        Commands::Print {
            path: register_path,
        } => absolem_core.print_shell_script(register_path),
        Commands::List => absolem_core.list_shell_scripts(),
        Commands::Config { command } => match command {
            ConfigCommand::View => absolem_core.config_view(),
            ConfigCommand::Set { key, value } => absolem_core.config_set(key, value),
            ConfigCommand::Delete { key } => absolem_core.config_delete(key),
        },
    }
}
