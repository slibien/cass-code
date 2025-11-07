use cass_app_server::run_main;
use cass_arg0::arg0_dispatch_or_else;
use cass_common::CliConfigOverrides;

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|cass_linux_sandbox_exe| async move {
        run_main(cass_linux_sandbox_exe, CliConfigOverrides::default()).await?;
        Ok(())
    })
}
