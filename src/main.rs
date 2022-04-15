mod commands;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Name of the shellcode. Eg, "bash_tcp"
    #[clap(short, long)]
    shellcode_type: Option<String>,
    /// IP address for the reverse shell to connect back to (eg, 10.10.10.20)
    #[clap(short, long)]
    ip: Option<String>,
    /// Port that your netcat listener is listening on (eg, 4444)
    #[clap(short, long)]
    port: Option<u16>,
    /// Shell in use
    #[clap(long, default_value_t=String::from("/bin/sh"))]
    shell: String,

    /// List all reverse shell names in the database.
    #[clap(long)]
    list: bool,
    /// List all reverse shells AND their commands (WARNING, long)
    #[clap(long)]
    list_commands: bool,
    /// User wants to list the raw generation for their chosen revshell
    #[clap(long, requires="shellcode-type")]
    print_command: bool,
}

fn main() {
    let args = Args::parse();
    // If user wants to list the shell names, then do that
    if args.list {
        print_shells();
    }
    // If they want to preview the commands
    else if args.list_commands {
        print_shells_and_commands();
    }
    else if args.print_command {
        list_command(args);
    }
    else {
        // Otherwise, generate the shell
        println!("Your Generated Shell");
        println!("{}", subsitute_variables(args));
    }
}

fn subsitute_variables(args: Args) -> String {
    if args.ip.is_none() {
        eprintln!("Please specify the reverse shell to generate");
        panic!();
    }
    if args.ip.is_none() {
        eprintln!("Please specify your host IP.");
        panic!();
    }
    if args.port.is_none() {
        eprintln!("Please specify your listening port.");
        panic!();
    }
    let command = commands::COMMANDLIST.get(&args.shellcode_type.unwrap()).unwrap();
    let sub = command.replace("{SUBIP}", &args.ip.unwrap());
    let sub = sub.replace("{SUBPORT}", &args.port.unwrap().to_string());
    sub
}

fn print_shells() {
    for (shellname, _) in &*commands::COMMANDLIST {
        println!("{}", shellname);
    }
}

fn print_shells_and_commands() {
    for (shellname, shellcmd) in &*commands::COMMANDLIST {
        println!("\n{}:\n\t {}", shellname, shellcmd);
    }
}

fn list_command(args: Args) {
    let x = commands::COMMANDLIST.get(&args.shellcode_type.unwrap()).expect("No reverse shell with that name exists!");
    println!("{}", x)
}
