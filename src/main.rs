use anyhow::{Context, Ok};
use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use std::ffi::CStr;
use std::io::prelude::*;
use std::{fs, io::BufReader};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Doc comment
#[derive(Debug, Subcommand)]
enum Command {
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,

        object_hash: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory");
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            let mut f = std::fs::File::open(format!(
                ".git/object/{}/{}",
                &object_hash[..2],
                &object_hash[2..]
            ))
            .context("opne in .git/objects")?;
            let mut z = ZlibDecoder::new(f);
            let mut z = BufReader::new(z);
            let mut buf = Vec::new();
            z.read_until(0, &mut buf)
                .context("read header from .git/objects")?;
            let header = CStr::from_bytes_with_nul(&buf).expect("Know there is exactly one nuk, and it's at the end");
            let header = header.to_str().context(".git/objects file header isn't valid UTF-8")?;
            let Some(size) = header.strip_prefix("blob") else {
                anyhow::bail!(".git/objects file header didn't start with 'blob ': '{header}'")
            };
            let size = size.parse::<usize>().context(
                ".git/objects file header has valid size: {size}"
            )?;
            buf.clear();
            buf.resize(size, 0);
            z.read_exact(&mut buf[..]).context("read true contents of .git/objects file");
            let n = z.read(&mut [0]).context("validate EOF in .git/object file")?;

            anyhow::ensure!(n == 0, ".git/object file had {n} trailing bytes");

            let mut stdout = std::io::stdout().lock();
            stdout.write_all(&buf).context("write objects to stdout");
        }   
    }
    Ok(())
}
