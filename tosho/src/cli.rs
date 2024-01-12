use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser, Subcommand,
};

use crate::r#impl::{kmkc::KMKCCommands, musq::MUSQCommands, tools::ToolsCommands};

pub(crate) type ExitCode = u32;

#[derive(Parser)]
#[command(name = "tosho")]
#[command(bin_name = "tosho")]
#[command(author, version, about, long_about = None, styles = cli_styles())]
#[command(propagate_version = true, disable_help_subcommand = true)]
pub(crate) struct ToshoCli {
    /// Increase message verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,

    #[command(subcommand)]
    pub(crate) command: ToshoCommands,
}

#[derive(Subcommand)]
pub(crate) enum ToshoCommands {
    /// Download manga from MU!
    #[command(name = "mu")]
    Musq {
        #[command(subcommand)]
        subcommand: MUSQCommands,
    },
    /// Download manga from KM
    #[command(name = "km")]
    Kmkc {
        #[command(subcommand)]
        subcommand: KMKCCommands,
    },
    Tools {
        #[command(subcommand)]
        subcommand: ToolsCommands,
    },
}

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Magenta.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::BrightCyan.on_default())
}
