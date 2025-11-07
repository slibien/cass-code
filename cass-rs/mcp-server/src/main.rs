use cass_arg0::arg0_dispatch_or_else;
use cass_common::CliConfigOverrides;
use cass_mcp_server::run_main;

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|cass_linux_sandbox_exe| async move {
        run_main(cass_linux_sandbox_exe, CliConfigOverrides::default()).await?;
        Ok(())
    })
}
