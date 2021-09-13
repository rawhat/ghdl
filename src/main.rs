use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use structopt::StructOpt;

#[derive(Debug, Deserialize)]
struct ContentsResponse {
    path: String,
    git_url: String,
}

#[derive(Debug, Deserialize)]
struct TreeEntry {
    path: String,
    #[serde(rename(deserialize = "type", serialize = "entry_type"))]
    entry_type: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct TreeResponse {
    tree: Vec<TreeEntry>,
}

#[derive(Debug, Deserialize)]
struct BlobResponse {
    content: String,
}

/// Tool to pull down specific paths from a github repo
#[derive(StructOpt, Debug)]
struct Opt {
    /// Optional* GitHub API key (you will get rate-limited if you don't specify one)
    #[structopt(long)]
    api_key: Option<String>,

    /// Branch to pluck folder from
    #[structopt(short, long)]
    branch: String,

    /// GitHub repo to pull from
    #[structopt(short, long)]
    repo: String,

    /// Path to file or folder to pull down
    #[structopt(name = "repo path")]
    folder: String,

    /// Location to save to, defaulting to $PWD
    #[structopt(name = "output dir", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opts = Opt::from_args();

    let mut path = PathBuf::new();
    path.push(opts.folder.clone());
    let path = path.clone();
    let parent_folder = path.parent().unwrap_or_else(|| Path::new("."));

    let contents_url = format!(
        "https://api.github.com/repos/{}/contents/{}",
        opts.repo,
        parent_folder.to_str().unwrap()
    );

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("vnd.github.v3+json"));
    if let Some(ref api_token) = opts.api_key {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(format!("token {}", &api_token).as_str()).unwrap(),
        );
    }
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

    let client = Client::new();
    let contents = client
        .get(contents_url)
        .headers(headers.clone())
        .send()?
        .json::<Vec<ContentsResponse>>()?;

    let entry_path = match contents.iter().find(|resp| resp.path == opts.folder) {
        Some(ContentsResponse { git_url, .. }) => git_url.clone(),
        _ => bail!("Provided folder not found in repository"),
    };

    let tree_entries = client
        .get(entry_path)
        .query(&[("recursive", "true")])
        .headers(headers.clone())
        .send()?
        .json::<TreeResponse>()?
        .tree;

    let output_base = opts.output.unwrap_or_else(|| env::current_dir().unwrap());

    let tree_files = tree_entries
        .iter()
        .filter(|e| e.entry_type != "tree")
        .collect::<Vec<&TreeEntry>>();
    let total_files = tree_files.len();

    for (index, entry) in tree_files.iter().enumerate() {
        println!(
            "Downloading {} ({} of {})...",
            entry.path,
            index + 1,
            total_files
        );

        let full_path = output_base.clone().join(entry.path.clone());
        let parent = full_path.parent().unwrap();
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }

        let blob = client
            .get(entry.url.clone())
            .headers(headers.clone())
            .send()?
            .json::<BlobResponse>()?
            .content
            .replace("\n", "");

        let contents = match base64::decode(blob) {
            Ok(contents) => contents,
            _ => {
                println!("Failed to decode blob...");
                continue;
            }
        };

        fs::write(full_path.to_str().unwrap(), contents)
            .with_context(|| "Could not write contents to file")?;
    }

    Ok(())
}
