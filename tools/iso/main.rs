use std::{path::PathBuf, process::Command};

/// ISO creating utility
#[derive(argp::FromArgs)]
struct Cli {
    /// The output iso
    #[argp(positional)]
    out: PathBuf,
    /// Executable
    #[argp(positional)]
    exec: PathBuf,
    /// The apploader program
    #[argp(positional)]
    apploader: PathBuf,
    /// The files to include in the ISO
    #[argp(positional)]
    fs: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli: Cli = argp::parse_args_or_exit(&argp::HelpStyle::default());

    let disk_dir = tempdir::TempDir::new("tools-iso-packaging")?.into_path();

    for f in cli.fs {
        std::fs::write(disk_dir.join(f.file_name().unwrap()), std::fs::read(f)?)?;
    }

    let exec_name = cli.exec.file_name().unwrap();

    if !Command::new("./dollz3")
        .arg(&cli.exec)
        .arg(disk_dir.join(exec_name))
        .arg("-m")
        .status()?
        .success()
    {
        anyhow::bail!("failed to compress executable with dollz3")
    }

    if !Command::new("./mkisofs")
        .arg("-R")
        .arg("-J")
        .arg("-G")
        .arg(cli.apploader)
        .arg("-no-emul-boot")
        .arg("-b")
        .arg(exec_name)
        .arg("-o")
        .arg(cli.out)
        .arg(&disk_dir)
        .status()?
        .success()
    {
        anyhow::bail!("failed to run mkisofs")
    }

    std::fs::remove_dir_all(disk_dir)?;

    Ok(())
}
