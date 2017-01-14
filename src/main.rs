#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate git2;
extern crate prettytable;

use clap::{App, Arg, ArgMatches};
use git2::Repository;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use error::{ErrorKind, Result};

mod error;

pub fn exit(message: &str) -> ! {
    let err = clap::Error::with_description(message, clap::ErrorKind::InvalidValue);
    err.exit();
}

pub fn repo_root(repo: &Repository) -> Result<PathBuf> {
    match repo.workdir() {
        Some(path) => Ok(path.to_owned()),
        None => Err(ErrorKind::RepoEmpty.into())
    }
}

pub fn branches_from_file(path: &PathBuf) -> Result<Vec<String>> {
    let mut file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut res = Vec::new();
    for line in buf.lines() {
        res.push(match line {
            Ok(line) => line,
            Err(_) => exit("Unparseable .amwy file")
        });
    }
    Ok(res)
}

pub fn branches_from_repo(repo: &Repository) -> Result<Vec<String>> {

}

pub fn check_branches(repo: &Repository) -> Result<Vec<String>> {
    let mut branch_file_path = repo_root(repo)?;
    branch_file_path.push(".awmy");
    let branches = match branches_from_file(&branch_file_path) {
        Ok(branches) => branches,
        Err(e) => return Err(e)
    };
    Err(ErrorKind::RepoEmpty.into())
}

fn main() {
    let yml = load_yaml!("app.yml");
    let matches = App::from_yaml(yml)
        .author(crate_authors!())
        .version(crate_version!())
        .get_matches();

    let current_dir = env::current_dir().unwrap();

    let repo = match Repository::discover(current_dir) {
        Ok(repo) => repo,
        Err(_) => exit("Not in a git repository")
    };

}
