use expectrl::{spawn, Eof, Regex, WaitStatus};

fn main() {
    let mut p = spawn("ftp speedtest4.tele2.net").unwrap();
    p.expect(Regex("Name \\(.*\\):")).unwrap();
    p.send_line("anonymous").unwrap();
    p.expect("Password").unwrap();
    p.send_line("test").unwrap();
    p.expect("ftp>").unwrap();
    p.send_line("cd upload").unwrap();
    p.expect("successfully changed.\r\nftp>").unwrap();
    p.send_line("pwd").unwrap();
    p.expect(Regex("[0-9]+ \"/upload\"")).unwrap();
    p.send_line("exit").unwrap();
    p.expect(Eof).unwrap();
    assert_eq!(p.wait().unwrap(), WaitStatus::Exited(p.pid(), 0));
}
