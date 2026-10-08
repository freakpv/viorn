extern crate getopts;

use anyhow::Context;
use args::Args;
use getopts::Occur;

fn main() -> anyhow::Result<()> {
    let mut args = Args::new("viorn", "traffic filter");
    args.flag("H", "help", "Prints this help information");
    args.flag("V", "version", "Version information about the binary");
    args.option(
        "C",
        "config",
        "Path to the configuration .toml file",
        "",
        Occur::Optional,
        None,
    );

    args.parse_from_cli()?;

    let help = args.value_of("help")?;
    if help {
        println!("{}", args.full_usage());
        return Ok(());
    }

    let version = args.value_of("version")?;
    if version {
        const SHA: Option<&str> = option_env!("GIT_HASH");
        const VER: Option<&str> = option_env!("CARGO_PKG_VERSION");
        println!(
            "traffic_filter: version:{} git-sha:{}",
            VER.unwrap_or("unknown"),
            SHA.unwrap_or("unknown")
        );
        return Ok(());
    }

    args.value_of::<String>("config")
        .map_err(Into::into)
        .and_then(|config| {
            std::fs::read_to_string(&config)
                .with_context(|| format!("Failed to read config {}", config))
                .map_err(Into::into)
        })
        .and_then(|config| config.parse::<toml::Table>().map_err(Into::into))
        //.and_then(|config| config.get_table("rules").map_err(Into::into))
        .and_then(|_| Ok(()))
}
