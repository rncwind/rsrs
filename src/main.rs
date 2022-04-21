mod commands;

use clap::{Parser, Subcommand, Args};
use commands::{ SHELLS, Encoding, RevShell };

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommands
}

#[derive(Args, Debug, Clone)]
struct GenerateArgs {
    /// Name of the shellcode. Eg, "bash_tcp"
    #[clap(short, long)]
    revshell: Option<String>,
    /// IP address for the reverse shell to connect back to (eg, 10.10.10.20)
    #[clap(short, long)]
    ip: Option<String>,
    /// Port that your netcat listener is listening on (eg, 4444)
    #[clap(short, long)]
    port: Option<u16>,
    /// Shell in use
    #[clap(long, default_value_t=String::from("/bin/sh"))]
    shell: String,
    /// Encoding to output the shell with
    #[clap(arg_enum, default_value_t=Encoding::None, long, short)]
    encoding: Encoding,
}

#[derive(Subcommand, Debug, Clone)]
enum SubCommands {
    Generate(GenerateArgs),
    ListShells{filter: Option<String>},
    ListShellsVerbose{filter: Option<String>},
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        SubCommands::Generate(generateArgs) => {
            let res = generate_shell(&generateArgs);
            match res {
                Ok(rs) => {
                    println!("{}", rs);
                } Err(x) => {
                    panic!("{}", x);
                }
            }
        },
        SubCommands::ListShells { filter } => {
            list_shells(filter);
        },
        SubCommands::ListShellsVerbose{filter} => {
            list_shells_verbose(filter);
        },
    }
}

fn generate_shell(args: &GenerateArgs) -> Result<String, String> {
    let shell = args.revshell.clone().expect("Please enter a valid reverse shell name");
    let pickedshell = SHELLS.get(&shell).expect("Please provide a valid shell!");
    match validate_components(args, pickedshell) {
        Ok(_) => {
            let subbed = substitute_components(args.clone(), pickedshell.clone());
            return Ok(subbed);
        },
        Err(emsg) => {
            return Err(emsg);
        },
    }
}

/// Ensures that the required command line arguments were passed to fufill
/// the set of substitutions we need, in order to generate a valid reverse
/// shell.
fn validate_components(args: &GenerateArgs, rs: &RevShell) -> Result<(), String> {
    if args.ip.is_none() & rs.sub_components.contains(&"IP".to_string()) {
        return Err("IP Flag is Required".to_string());
    }
    if args.port.is_none() & rs.sub_components.contains(&"PORT".to_string()) {
        return Err("Port Flag is Required".to_string());
    }
    Ok(())
}

/// Substitute the components of the reverse shell that we need to specialise
/// in order to be of use to any would-be red-teamer.
fn substitute_components(args: GenerateArgs, rs: RevShell) -> String {
    let mut modified = rs.command.clone();
    if rs.sub_components.contains(&"IP".to_string()) {
        modified = modified.replace("{SUBIP}", &args.ip.expect("Requires a valid ip!"));
    }
    if rs.sub_components.contains(&"PORT".to_string()) {
        modified = modified.replace("{SUBPORT}", &args.port.expect("Requires a valid port!").to_string());
    }
    if rs.sub_components.contains(&"SHELL".to_string()) {
        modified = modified.replace("{SUBSHELL}", &args.shell);
    }
    encode(args.encoding, modified)
}

/// Given a fully substituted string, apply one of the encoding steps that we
/// support.
fn encode(encoding: Encoding, completers: String) -> String {
    match encoding {
        Encoding::None => {
            return completers;
        },
        Encoding::Url => {
            return urlencoding::encode(&completers).to_string();
        },
        Encoding::DoubleUrl => {
            return urlencoding::encode(&urlencoding::encode(&completers).to_string()).to_string();
        },
        Encoding::Base64 => {
            return base64::encode(completers);
        }
    }
}

/// List all shells, their componeents, their raw string, and what OS they support.
fn list_shells_verbose(filter: Option<String>) {
    match filter {
        Some(filter) => {
            for(_,shell) in &*SHELLS {
                if shell.os_support.contains(&filter) {
                    println!("{}", shell);
                }
            }
        },
        None => {
            for(_, shell) in &*SHELLS {
                println!("{}\n", shell);
            }
        }
    }
}

fn list_shells(filter: Option<String>) {
    match filter {
        Some(x) => {
            for(shell, shellinfo) in &*SHELLS {
                if shellinfo.os_support.contains(&x) {
                    println!("{}", shell);
                }
            }
        },
        None => {
            for (shell, _) in &*SHELLS {
                println!("{}", shell);
            }
        }
    }
}
