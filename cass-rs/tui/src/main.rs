use clap::Parser;
use cass_arg0::arg0_dispatch_or_else;
use cass_common::CliConfigOverrides;
use cass_tui::Cli;
use cass_tui::run_main;

#[derive(Parser, Debug)]
struct TopCli {
    #[clap(flatten)]
    config_overrides: CliConfigOverrides,

    #[clap(flatten)]
    inner: Cli,
}

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|cass_linux_sandbox_exe| async move {
        let top_cli = TopCli::parse();
        let mut inner = top_cli.inner;
        inner
            .config_overrides
            .raw_overrides
            .splice(0..0, top_cli.config_overrides.raw_overrides);
        let exit_info = run_main(inner, cass_linux_sandbox_exe).await?;
        let token_usage = exit_info.token_usage;
        if !token_usage.is_zero() {
            println!("{}", cass_core::protocol::FinalOutput::from(token_usage),);
        }
        Ok(())
    })
}
