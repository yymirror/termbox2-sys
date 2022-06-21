use std::env;
use std::fs;
use std::path::Path;
use std::process::{Stdio, Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dst = Path::new(&out_dir);

    build();
    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=termbox2");
}

fn build() {
    let mut cmd = mkslib();
    env::set_var("DESTDIR", env::var("OUT_DIR").unwrap());
    run(&mut cmd);
    env::remove_var("DESTDIR");
}

fn mkslib() -> Command {
    let cwd = env::current_dir().expect("Couldn't get the current directory");
    let workdir = Path::new(&cwd);
    let cmd_file = fs::canonicalize(&workdir.join("mkslib.sh")).expect("Couldn't find mkslib.sh-script");
    let mut cmd = Command::new(cmd_file);
    cmd.current_dir(&workdir);
    cmd
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap()
                .success());
}
