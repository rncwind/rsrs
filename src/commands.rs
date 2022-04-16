use std::collections::HashMap;
use serde::Deserialize;
use lazy_static::lazy_static;
use std::fmt;

lazy_static! {
    /// This builds the set of possible reverse shells from revshells.json
    /// at compile time. When the program starts up, this hashtable of shells
    /// is built at first access.
    pub static ref SHELLS: HashMap<String, RevShell> = {
        let mut m = HashMap::new();
        let raw_json = std::include_str!("../revshells.json");
        let shells: Vec<RevShell> = serde_json::from_str(&raw_json).expect("Failed to parse revshells.json");
        for shell in shells {
            m.insert(shell.name.clone(), shell.clone());
        }
        m
    };
}


#[derive(Deserialize, Debug, Clone)]
pub struct RevShell {
    /// The name of the reverse shell
    pub name: String,
    /// The shell command that we will modify in this generator
    pub command: String,
    /// What Opperating Systems does this reverse shell work on?
    pub os_support: Vec<String>,
    /// What values do we need to get from the user, to generate this reverse shell.
    pub sub_components: Vec<String>,
}

impl fmt::Display for RevShell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name:\t\t{}\nOS:\t\t{:?}\nSubcomponents:\t{:?}\nCommand:\t{}", self.name, self.os_support, self.sub_components, self.command)
    }
}

/// Enumerate encoding options. Default is None
#[derive(Debug, clap::ArgEnum, Clone)]
pub enum Encoding {
    None, UrlEncode, DoubleUrlEncode, Base64Encode,
}
