<p align="center"><code>npm i -g @orion-ai/cass-code</code><br />or <code>brew install --cask cass</code></p>

<p align="center"><strong>Cass Code</strong> is an autonomous AI coding assistant powered by Claude that runs locally on your computer.
</br>
</br>Cass Code combines the power of Orion AI's Claude with advanced features like subagents, plugins, and Model Context Protocol (MCP) integration.</p>

<p align="center">
  <img src="./.github/cass-cli-splash.png" alt="Cass Code splash" width="80%" />
  </p>

---

## Quickstart

### Installing and running Cass Code

Install globally with your preferred package manager. If you use npm:

```shell
npm install -g @orion-ai/cass-code
```

Alternatively, if you use Homebrew:

```shell
brew install --cask cass
```

Then simply run `cass` to get started:

```shell
cass
```

If you're running into upgrade issues with Homebrew, see the [FAQ entry on brew upgrade cass](./docs/faq.md#brew-upgrade-cass-isnt-upgrading-me).

<details>
<summary>You can also go to the <a href="https://github.com/slibien/cass-code/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `cass-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `cass-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `cass-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `cass-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `cass-x86_64-unknown-linux-musl`), so you likely want to rename it to `cass` after extracting it.

</details>

### Using Cass Code with Claude

<p align="center">
  <img src="./.github/cass-cli-login.png" alt="Cass Code login" width="80%" />
  </p>

Run `cass` and authenticate with your Orion AI API key or Claude Pro/Max subscription. Cass Code is powered by Claude Sonnet 4.5 by default, with options to use Opus for complex reasoning or Haiku for speed.

For API key setup, see [authentication docs](./docs/authentication.md). If you're having trouble with login, please check the [authentication guide](./docs/authentication.md).

### Model Context Protocol (MCP)

Cass Code has deep MCP integration, allowing connection to external tools, databases, and APIs. To configure MCP servers, refer to the [config docs](./docs/config.md#mcp_servers).

### Configuration

Cass Code supports a rich set of configuration options, with preferences stored in `~/.cass/config.toml`. For full configuration options, see [Configuration](./docs/config.md).

---

### Docs & FAQ

- [**Getting started**](./docs/getting-started.md)
  - [CLI usage](./docs/getting-started.md#cli-usage)
  - [Slash Commands](./docs/slash_commands.md)
  - [Running with a prompt as input](./docs/getting-started.md#running-with-a-prompt-as-input)
  - [Example prompts](./docs/getting-started.md#example-prompts)
  - [Custom prompts](./docs/prompts.md)
  - [Memory with AGENTS.md](./docs/getting-started.md#memory-with-agentsmd)
- [**Configuration**](./docs/config.md)
  - [Example config](./docs/example-config.md)
- [**Sandbox & approvals**](./docs/sandbox.md)
- [**Authentication**](./docs/authentication.md)
  - [Auth methods](./docs/authentication.md#forcing-a-specific-auth-method-advanced)
  - [Login on a "Headless" machine](./docs/authentication.md#connecting-on-a-headless-machine)
- **Automating Cass Code**
  - [GitHub Action](https://github.com/slibien/cass-code-action)
  - [TypeScript SDK](./sdk/typescript/README.md)
  - [Non-interactive mode (`cass exec`)](./docs/exec.md)
- [**Advanced**](./docs/advanced.md)
  - [Tracing / verbose logging](./docs/advanced.md#tracing--verbose-logging)
  - [Model Context Protocol (MCP)](./docs/advanced.md#model-context-protocol-mcp)
- [**Zero data retention (ZDR)**](./docs/zdr.md)
- [**Contributing**](./docs/contributing.md)
- [**Install & build**](./docs/install.md)
  - [System Requirements](./docs/install.md#system-requirements)
  - [DotSlash](./docs/install.md#dotslash)
  - [Build from source](./docs/install.md#build-from-source)
- [**FAQ**](./docs/faq.md)
- [**Open source fund**](./docs/open-source-fund.md)

---

## License

This repository is licensed under the [Apache-2.0 License](LICENSE).
