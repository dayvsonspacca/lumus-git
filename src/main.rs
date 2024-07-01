use clap::{Parser, Subcommand};
use cli_table::{print_stdout, Table, WithTitle};
use git2::Repository;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(version, about = "GitHub insights locally", long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show contributors
    Contributors,
}

#[derive(Table)]
struct ContributorRow {
    #[table(title = "Author")]
    author: String,
    #[table(title = "Commits")]
    commits: usize,
    #[table(title = "E-mail")]
    email: String,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Contributors => show_contributors(),
    }
}

fn show_contributors() {
    let repo = Repository::open(".").expect("Failed to open repository");

    let mut revwalk = repo.revwalk().expect("Failed to create revision walker");
    revwalk.push_head().expect("Failed to push head");

    let mut authors: HashMap<String, ContributorRow> = HashMap::new();

    revwalk
        .filter_map(|id| id.ok())
        .filter_map(|id| repo.find_commit(id).ok())
        .for_each(|commit| {
            let author = commit.author();
            let name = author.name().unwrap_or("Unknown").to_string();
            let email = author.email().unwrap_or("").to_string();

            let entry = authors
                .entry(name.clone())
                .or_insert_with(|| ContributorRow {
                    author: name.clone(),
                    commits: 0,
                    email: email.clone(),
                });

            entry.commits += 1;
        });

    let mut table: Vec<ContributorRow> = authors.into_values().collect();
    table.sort_by_key(|row| std::cmp::Reverse(row.commits));

    print_stdout(table.with_title()).expect("Failed to print table");
}