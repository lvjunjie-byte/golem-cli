// Copyright 2024-2025 Golem Cloud
//
// Licensed under the Golem Source License v1.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://license.golem.cloud/LICENSE
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::hooks::NoHooks;
use golem_cli::command_handler::CommandHandler;
use std::process::ExitCode;
use std::sync::Arc;

#[cfg(feature = "server-commands")]
mod hooks {
    use golem_cli::command::server::ServerSubcommand;
    use golem_cli::command_handler::CommandHandlerHooks;
    use golem_cli::context::Context;

    use clap_verbosity_flag::Verbosity;
    use std::sync::Arc;
    use anyhow::Context as AnyhowContext;
    use tracing::info;

    pub struct NoHooks {}

    impl CommandHandlerHooks for NoHooks {
        #[cfg(feature = "server-commands")]
        async fn handler_server_commands(
            &self,
            ctx: Arc<Context>,
            subcommand: ServerSubcommand,
        ) -> anyhow::Result<()> {
            match subcommand {
                ServerSubcommand::Run { args } => {
                    // Existing server run logic (placeholder)
                    info!("Running golem server: {:?}", args);
                    Ok(())
                }
                ServerSubcommand::Clean => {
                    // Existing server clean logic (placeholder)
                    info!("Cleaning server data");
                    Ok(())
                }
                #[cfg(feature = "mcp-server")]
                ServerSubcommand::Mcp { port, http } => {
                    use golem_cli::command_handler::server::mcp_server;
                    
                    if http {
                        info!("Starting MCP server in HTTP mode on port {}", port);
                        mcp_server::start_mcp_server_http(ctx, port).await
                            .context("MCP server failed")?;
                    } else {
                        info!("Starting MCP server in stdio mode");
                        mcp_server::start_mcp_server_stdio(ctx).await
                            .context("MCP server failed")?;
                    }
                    Ok(())
                }
            }
        }

        #[cfg(feature = "server-commands")]
        async fn run_server() -> anyhow::Result<()> {
            // Placeholder for auto-start server logic
            Ok(())
        }

        #[cfg(feature = "server-commands")]
        fn override_verbosity(verbosity: Verbosity) -> Verbosity {
            verbosity
        }

        #[cfg(feature = "server-commands")]
        fn override_pretty_mode() -> bool {
            false
        }
    }
}

#[cfg(not(feature = "server-commands"))]
mod hooks {
    use golem_cli::command_handler::CommandHandlerHooks;

    pub struct NoHooks {}

    impl CommandHandlerHooks for NoHooks {}
}

fn main() -> ExitCode {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build tokio runtime for golem-cli main")
        .block_on(CommandHandler::handle_args(
            std::env::args_os(),
            Arc::new(NoHooks {}),
        ))
}
