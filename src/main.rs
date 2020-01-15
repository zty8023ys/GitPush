extern crate clap;

use clap::{App, Arg};
use std::process::Command;
use std::ffi::OsStr;

fn main() {
    let app = App::new("GitPush")
        .version("1.0")
        .author("Ztory <Ztory@foxmail.com>")
        .about("Auto upload current local branch to all remote repository")
        .usage("gp push")
        .arg(Arg::with_name("push")
            .help("Do it")
            .empty_values(true)
        );
    let matches = app.get_matches();

    if match matches.value_of("push") {
        Some(r) => r,
        None => ""
    } == "" {
        return;
    }
    let repos_str = get_repos();
    let repos = no_empty_str_arr(&repos_str);
    let branch = get_current_branch();
    for repo in repos {
        let str = push(&repo, &branch);
        println!("{}",str);
    }
}

fn get_current_branch() -> String {
    run("git", vec!["rev-parse", "--abbrev-ref", "HEAD"]).replace("\n", "")
}

fn get_repos() -> String {
    let str = run("git", vec!["remote"]);
    str
}

fn run<I, S>(cmd: S, args: I) -> String
    where
        I: IntoIterator<Item=S>,
        I: std::fmt::Debug,
        S: AsRef<OsStr>,
        S: std::fmt::Debug
{
    let output = Command::new(cmd)
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!(err.to_string()));
    if output.stderr.len() > 0 {
        return String::from_utf8_lossy(&output.stderr).to_string();
    }
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn push(repo: &str, branch: &str) -> String {
    run("git", vec!["push", repo, branch])
}

fn no_empty_str_arr(str1: &String) -> Vec<&str> {
    str1
        .split("\n").collect::<Vec<&str>>()
        .into_iter().filter(|b| *b != "").collect::<Vec<&str>>()
}