mod commands;

use clap::Parser;
use commands::{ SHELLS, Encoding, RevShell };

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
struct Args {
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
    #[clap(arg_enum, default_value_t=Encoding::None)]
    encoding: Encoding,

    /// List all reverse shell names in the database.
    #[clap(long)]
    list_shells: bool,
    /// List all reverse shells AND their commands (WARNING, long)
    #[clap(long)]
    list_shells_verbose: bool,
}

fn main() {
    let args = Args::parse();
    // If user wants to list the shell names, then do that
    if args.list_shells {
        list_shells()
    }
    // If they want to preview the commands
    else if args.list_shells_verbose {
        list_shells_verbose();
    }
    else {
        let sc = args.revshell.clone().expect("Please give a valid reverse shell name");
        let pickedshell = SHELLS.get(&sc.clone()).expect("Please provide a valid shell!").clone();
        match validate_components(&args, &pickedshell) {
            Ok(()) => {
                println!("{}", substitute_components(args, pickedshell));
            },
            Err(x) => {
                panic!("{}", x);
            }
        }
        // Otherwise, generate the shell
    }
}

fn validate_components(args: &Args, rs: &RevShell) -> Result<(), String> {
    if args.ip.is_none() & rs.sub_components.contains(&"IP".to_string()) {
        return Err("IP Flag is Required".to_string());
    }
    if args.port.is_none() & rs.sub_components.contains(&"PORT".to_string()) {
        return Err("Port Flag is Required".to_string());
    }
    Ok(())
}

fn substitute_components(args: Args, rs: RevShell) -> String {
    let mut modified = rs.command.clone();
    if rs.sub_components.contains(&"IP".to_string()) {
        modified = modified.replace("{SUBIP}", &args.ip.expect("Requires a valid ip!"));
    }
    if rs.sub_components.contains(&"PORT".to_string()) {
        modified = modified.replace("{SUBPORT}", &args.port.expect("Requires a valid port!").to_string());
    }
    modified
}

fn list_shells() {
    for (shell, _) in &*SHELLS {
        println!("{}", shell);
    }
}

fn list_shells_verbose() {
    for(_, shell) in &*SHELLS {
        println!("{}\n", shell);
    }
}
