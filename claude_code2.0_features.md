# Claude Code 2.0: Complete Feature Reference

**Claude Code 2.0** (also known as "cass code") is an autonomous AI coding assistant released by Orion AI on September 29, 2025. Unlike traditional code completion tools, Claude Code operates as an intelligent agent that understands high-level goals, plans multi-step workflows, and executes complex development tasks. This comprehensive reference documents every feature available in Claude Code 2.0, from basic slash commands to advanced plugin ecosystems and MCP integrations.

## Core capabilities and architecture

Claude Code 2.0 runs as both a **command-line interface** and a **VS Code extension**, powered by Claude Sonnet 4.5 by default. The tool maintains autonomous decision-making capabilities while requiring user approval for file modifications and command execution, creating a balance between automation and safety. Version 2.0 introduced revolutionary features including automatic checkpoints with instant rollback, specialized subagents for parallel workflows, an extensible plugin marketplace, and deep integration with the Model Context Protocol for connecting to external tools and data sources.

Installation requires Node.js 18+ and is accomplished via `npm install -g @orion-ai/claude-code`. Authentication works through Claude Pro/Max subscriptions or API keys from the Orion AI Console.

## All slash commands

Claude Code 2.0 provides an extensive command system accessible through the `/` prefix. Commands fall into three categories: built-in system commands, custom user-defined commands, and MCP-provided commands from integrated servers.

### Built-in system commands

**`/add-dir <directory-path>`** adds additional working directories to the current session, enabling Claude to work across multiple repositories or project folders simultaneously. When working on microservices or related projects, this command expands Claude's operational scope without restarting the session.

**`/agents`** opens the agent management interface for creating, configuring, and managing custom subagents. This interactive tool lets you define specialized AI assistants with specific roles, tool permissions, and system prompts. You can generate agents by describing their purpose to Claude or manually configure all settings including model selection, tool access, and visual themes.

**`/bug`** reports issues directly to Orion AI by sending current conversation data for debugging. This helps improve Claude Code by providing the development team with real-world error scenarios and edge cases.

**`/clear`** wipes conversation history to start fresh, critical for managing token consumption and maintaining clean context when switching between unrelated tasks. Regular use of this command prevents context pollution and keeps costs manageable during long development sessions.

**`/compact [instructions]`** summarizes the conversation to preserve important context while dramatically reducing token usage. Optional instructions focus the compaction on specific aspects. When approaching context window limits, this command condenses lengthy discussions into essential information, allowing work to continue without losing critical project details.

**`/config`** displays and modifies configuration settings through an interactive interface. View current settings with `/config` alone, or modify specific values with `/config set <key> <value>`. Configuration changes can affect model selection, permissions, notification preferences, and numerous other operational parameters.

**`/cost`** displays token usage statistics for the current session, helping monitor API consumption and manage expenses. This command shows cumulative token counts but has been largely superseded by the more comprehensive `/usage` command in version 2.0.

**`/doctor`** runs diagnostic checks on the Claude Code installation, verifying version information, configuration validity, environment setup, and connectivity. When troubleshooting issues or setting up new environments, this command identifies problems with dependencies, authentication, or system configuration.

**`/export`** saves the current conversation to a file or clipboard, available since version 1.0.44. This enables documentation of sessions, sharing conversations with team members, or creating records of AI-assisted development work for review or compliance purposes.

**`/help`** lists all available commands including built-in system commands, custom commands from `.claude/commands/`, and MCP server commands. Each command appears with its description, making this the primary discovery mechanism when you forget specific syntax or want to explore available functionality.

**`/init`** creates a CLAUDE.md file in the project root by scanning the codebase and generating starter project context. This file becomes the project's "constitution," documenting architecture, coding standards, common workflows, and essential context that Claude should know. A well-crafted CLAUDE.md dramatically improves Claude's effectiveness by reducing repetitive explanations.

**`/login`** initiates authentication with Claude Code, launching the OAuth flow for Pro/Max users or prompting for API key setup. Authentication persists across sessions until explicitly logged out.

**`/logout`** terminates the current authentication session, removing stored credentials.

**`/mcp`** manages Model Context Protocol servers through an interactive interface showing connected servers, authentication status, and health checks. Use this command to authenticate with OAuth-enabled remote servers, view available MCP resources, or troubleshoot connection issues.

**`/memory`** controls how Claude maintains context across sessions, managing conversation memory and project-specific knowledge persistence.

**`/model`** switches between Claude model variants. Options include **Opus** (most capable for complex reasoning, slower, higher cost), **Sonnet** (balanced performance and cost, the default in 2.0), and **Haiku** (fastest, lowest cost for simple tasks). Strategic model switching optimizes both performance and expenses.

**`/output-style`** views or selects custom output styles that transform how Claude presents information. Built-in styles include Default (standard software engineering), Explanatory (educational with detailed insights), and Learning (mentorship mode with TODO markers). Create custom styles with `/output-style:new` to completely replace the system prompt while maintaining all tool capabilities.

**`/permissions`** manages tool access control, showing which tools Claude can use and allowing configuration of allow/deny rules. This security-critical command prevents Claude from accessing sensitive files, executing dangerous commands, or using specific tools like WebFetch for environments without internet access.

**`/pr_comments`** manages GitHub pull request comment interactions, enabling workflows where Claude responds to PR review feedback.

**`/review`** requests code review with optional file targeting using `@` syntax like `/review @./src/components/Button.tsx`. Claude analyzes code quality, security vulnerabilities, performance issues, best practice violations, and test coverage, providing actionable feedback organized by priority.

**`/rewind`** activates the checkpoint rollback system, accessible via the command or by pressing Esc twice. This presents three restoration options: rewind conversation only (keeping code changes), revert code only (keeping conversation), or restore both to the previous checkpoint. Checkpoints are automatically created before each Claude edit, enabling safe exploration of multiple approaches without fear of losing work.

**`/status`** displays a comprehensive three-tab dashboard navigable with Tab key. The Status tab shows version, session ID, account information, current model, IDE integration status, and configuration sources. The Config tab reveals settings for auto-compact, checkpoints, theme, notifications, output style, editor mode, and model selection. The Usage tab displays current session percentage, weekly usage percentage, and Opus usage tracking with visual progress bars.

**`/usage`** shows plan limits and current consumption more comprehensively than `/cost`, displaying usage percentage, session ID, model information, and reset times adjusted for your timezone. This helps monitor whether you're approaching rate limits or plan thresholds.

### Custom slash commands

Custom commands are user-defined prompts stored as Markdown files that invoke like built-in commands. They provide reusable workflows, enforce team consistency, and capture institutional knowledge.

**Project-scoped commands** live in `.claude/commands/` and are shared with the team through version control. They appear in `/help` marked with "(project)" and establish consistent workflows across the entire team.

**User-scoped commands** reside in `~/.claude/commands/` and are personal to your account across all projects. They appear marked with "(user)" and enable personal productivity shortcuts that follow you between projects.

Command files use Markdown format with YAML frontmatter for metadata. The file name becomes the command name (dashes become the delimiter). Basic syntax is straightforward:

```markdown
---
description: Brief command description
---
Your command prompt goes here.
```

Advanced features include **argument passing** using `$ARGUMENTS` for all arguments or `$1`, `$2` for positional parameters. **Frontmatter options** control behavior: `allowed-tools` restricts which tools the command can use, `argument-hint` shows expected parameters in autocomplete, `model` specifies which Claude model to use, and `disable-model-invocation` prevents Claude from automatically executing the command.

**Shell command execution** within prompts uses the `!` prefix to run commands and inject their output into context:

```markdown
---
description: Create git commit with full context
---

Current status: !`git status`
Current diff: !`git diff HEAD`
Current branch: !`git branch --show-current`

Create an appropriate commit message and commit the changes.
```

**Namespaced commands** using subdirectories enable organization: `.claude/commands/frontend/component.md` becomes `/frontend:component`. This scales command libraries and prevents naming conflicts.

Example commands demonstrate power and flexibility. A `/fix-github-issue` command might use the `gh` CLI to fetch issue details, search the codebase for relevant files, implement changes, write tests, create commits, and open pull requests. A `/commit` command could run `git status` and `git diff`, determine the appropriate conventional commit type, stage changes, and push to the current branch. A `/optimize` command requests performance analysis focusing on algorithmic complexity, caching opportunities, and database efficiency.

### MCP-provided commands

MCP servers automatically expose their prompts as slash commands using the format `/mcp__servername__commandname`. For example, a GitHub MCP server might provide `/mcp__github__list_prs` to list pull requests or `/mcp__github__pr_review 456` to review a specific PR. These commands appear automatically in `/help` once the MCP server is configured and connected.

## Plugin system and marketplace

The plugin system introduced in Claude Code 2.0.13 (October 2025) transforms the tool into an extensible platform where developers package and share custom functionality through marketplace ecosystems.

### Plugin architecture

Plugins consist of four extension points that integrate seamlessly with Claude Code's core functionality.

**Slash commands** within plugins provide reusable operations. They're stored as Markdown files in the plugin's `commands/` directory and work identically to user-defined commands but are distributed through the plugin system.

**Subagents** are specialized AI assistants defined in the plugin's `agents/` directory. Each agent has its own context window, system prompt, tool permissions, and model selection. They enable delegation of specific tasks to purpose-built experts.

**Hooks** customize Claude Code's behavior at workflow inflection points. Defined in `hooks/hooks.json`, they trigger shell scripts at events like PreToolUse, PostToolUse, SubagentStop, or SessionStart. Uses include automated formatting, test enforcement, security validation, and logging.

**MCP servers** connect Claude to external tools and data sources. Plugins can bundle MCP server configurations in `.mcp.json`, making integrations instantly available when the plugin is enabled.

Standard plugin structure follows this layout:

```
plugin-name/
├── .claude-plugin/
│   └── plugin.json          # Required metadata
├── commands/                # Optional slash commands
│   ├── command1.md
│   └── command2.md
├── agents/                  # Optional specialized agents
│   ├── agent1.md
│   └── agent2.md
├── hooks/                   # Optional event hooks
│   └── hooks.json
├── skills/                  # Optional agent skills
│   └── skill-name/
│       └── SKILL.md
├── .mcp.json               # Optional MCP configuration
└── README.md
```

The `plugin.json` manifest specifies name, description, version, author information, homepage URL, repository location, license, and keywords for discoverability.

### Marketplace system

Marketplaces are JSON catalogs listing available plugins and their sources. The `marketplace.json` file defines marketplace metadata and plugin entries with names, sources (relative paths, GitHub repos, Git URLs, or local directories), descriptions, and versions.

Installing marketplaces extends Claude Code's plugin universe. Add GitHub repositories with `/plugin marketplace add owner/repo`, Git repositories with `/plugin marketplace add https://gitlab.com/team/plugin.git`, local directories with `/plugin marketplace add ./local-marketplace`, or remote URLs with `/plugin marketplace add https://url/marketplace.json`.

Management commands control marketplace access. List configured marketplaces with `/plugin marketplace list`, update a marketplace's plugin catalog with `/plugin marketplace update marketplace-name`, and remove marketplaces with `/plugin marketplace remove marketplace-name`.

### Plugin installation and management

Installing plugins follows either an interactive workflow or direct commands. The interactive method runs `/plugin` to open the browser, selects "Browse Plugins," chooses the desired plugin, clicks "Install now," and restarts Claude Code. Direct installation uses `/plugin install plugin-name@marketplace-name` for immediate installation.

Management commands control plugin lifecycle. List all installed plugins with `/plugin list`, enable a disabled plugin with `/plugin enable plugin-name`, temporarily disable a plugin with `/plugin disable plugin-name`, and completely remove a plugin with `/plugin remove plugin-name`.

Team collaboration involves committing plugin configuration to `.claude/settings.json`:

```json
{
  "extraKnownMarketplaces": {
    "team-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  },
  "enabledPlugins": [
    "plugin1@marketplace1",
    "plugin2@marketplace2"
  ]
}
```

When team members trust the folder, Claude Code automatically installs specified marketplaces and plugins, ensuring consistent tooling across the organization.

### Major plugin marketplaces

**Orion AI Official Plugins** at `anthropics/claude-code/plugins` provide foundational capabilities including compounding-engineering for multi-agent code review, agent-sdk for scaffolding Claude Agent SDK projects, and security-sentinel for security guidance.

**Claude Code Plugins Plus** by Jeremy Longshore (`jeremylongshore/claude-code-plugins-plus`) offers 227+ production-ready plugins across 15 categories including DevOps Automation Pack, Security Pro Pack, Fullstack Starter Pack, AI/ML Engineering Pack, and Excel Analyst Pro. It includes 167 Agent Skills compliant with Orion AI's v1.0 specification.

**Anand Tyagi's Marketplace** (`ananddtyagi/claude-code-marketplace`) focuses on community-driven commands and agents for code review, documentation, security, and testing. Notable plugins include Ultrathink (coordinator agent with 4 specialist sub-agents), Code Review Assistant, Documentation Generator, Bug Detective, and Lyra (prompt optimization specialist).

**Every Marketplace** (`EveryInc/every-marketplace`) emphasizes compounding engineering with 17 specialized review agents, multi-agent code review capabilities, systematic work execution, and GitHub issue integration.

**ClaudeCodeMarketplace.com** (https://claudecodemarketplace.com/) is a web-based directory with 140+ plugins in 14 specialized categories, featuring searchable navigation, installation guides, and detailed plugin information.

**Awesome Claude Code** (`hesreallyhim/awesome-claude-code`) curates commands, subagents, MCP servers, hooks, and workflows. Notable collections include AB Method (spec-driven workflow), SuperClaude (cognitive personas), Vibe-Log (session analysis), and Claude Code PM (project management).

### GitHub Spec Kit integration

GitHub Spec Kit is **not a Claude Code plugin** but rather a separate development methodology toolkit that works alongside Claude Code. Released by GitHub in 2025, Spec Kit shifts development from "vibe coding" to intent-driven development where specifications become executable artifacts rather than documentation to discard.

Installation uses uv: `uv tool install specify-cli --from git+https://github.com/github/spec-kit.git` or direct execution with `uvx --from git+https://github.com/github/spec-kit.git specify init <PROJECT_NAME>`.

The **Spec Kit workflow** consists of five phases that structure development from initial concept to implementation.

**Phase 1: Constitution** (`/speckit.constitution`) creates project governing principles and development guidelines in `.specify/memory/constitution.md`. This establishes the foundation for code quality standards, testing requirements, user experience consistency, and performance expectations.

**Phase 2: Specify** (`/speckit.specify`) captures requirements focusing on WHAT and WHY rather than technical details. The output includes user stories, functional requirements, acceptance criteria, and automatic Git branch creation for the feature.

**Phase 3: Plan** (`/speckit.plan`) adds technical architecture including tech stack selection, implementation strategy, database schemas, API specifications, and research on chosen technologies.

**Phase 4: Tasks** (`/speckit.tasks`) generates actionable task breakdown with dependency management, parallel execution markers, file path specifications, test-driven development structure, and checkpoint validation.

**Phase 5: Implement** (`/speckit.implement`) executes tasks systematically by validating prerequisites, parsing task breakdown, executing in dependency order, following TDD approaches, and providing progress updates.

Additional commands include `/speckit.clarify` for addressing underspecified areas before planning, `/speckit.analyze` for cross-artifact consistency analysis, and `/speckit.checklist` for quality validation checklists.

Spec Kit supports multiple AI agents including Claude Code, GitHub Copilot, Gemini CLI, Cursor, Windsurf, Qwen Code, opencode, Codex CLI, and Kilo Code, with limited support for Amazon Q.

The key distinction: Spec Kit is a methodology and toolkit (installed via CLI, distributed through Git) while Claude Code plugins are extensions (installed via `/plugin`, distributed through marketplaces). Spec Kit provides slash commands through `.claude/commands/` integration but operates independently as a development framework.

### Creating custom plugins

Building plugins starts with directory structure creation. Create folders for `.claude-plugin`, `commands`, `agents`, `hooks`, and `skills` as needed. Write the `plugin.json` manifest with metadata. Add slash commands as Markdown files with YAML frontmatter. Define subagents with name, description, tools, model, and system prompt. Configure hooks in `hooks/hooks.json` for lifecycle events. Add MCP servers in `.mcp.json` if external integrations are needed.

Testing locally requires creating a test marketplace with its own `marketplace.json` pointing to your plugin directory via relative path. Add the test marketplace with `/plugin marketplace add ./test-marketplace`, install your plugin with `/plugin install my-plugin@test-marketplace`, restart Claude Code, and test your commands with `/command-name`.

Publishing follows two paths. For GitHub repositories, create a repo, push plugin code, add `.claude-plugin/marketplace.json`, and users add it with `/plugin marketplace add username/repo`. For community marketplaces, fork the marketplace repository, add your plugin to marketplace.json, submit a pull request, and users automatically see your plugin in the existing marketplace.

## Subagents (Call Agents)

Subagents are specialized AI assistants that Claude Code delegates work to, each operating with independent context windows, custom system prompts, and scoped tool permissions. This enables modular, specialized workflows where different experts handle specific aspects of development.

### What subagents solve

Context pollution prevention keeps lengthy specialized tasks from cluttering the main conversation. Specialization enables domain-specific expertise in testing, security, architecture, documentation, or any other specialized area. Security and safety improve through scoped tool permissions where reviewers might have read-only access while implementers can write files. Reusability means agents can be version-controlled and shared across projects and teams. Workflow orchestration enables complex multi-step pipelines with clear separation of concerns.

### Technical implementation

Subagents are stored as Markdown files with YAML frontmatter in two hierarchical locations. Project-level agents in `.claude/agents/*.md` take highest priority and are shared through version control. User-level agents in `~/.claude/agents/*.md` are available across all projects but personal to your account.

File format includes required fields **name** (unique identifier using lowercase and hyphens) and **description** (natural language purpose description critical for automatic invocation). Optional fields include **tools** (comma-separated list of specific tools, inherits all tools if omitted) and **model** (sonnet, opus, or haiku, inherits from parent if omitted).

The system prompt follows the frontmatter and defines the agent's role, capabilities, approach, specific instructions, best practices, and constraints.

Example code reviewer agent:

```markdown
---
name: code-reviewer
description: Expert code review specialist. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: opus
---

You are a senior code reviewer ensuring high standards.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code simplicity and readability
- Function and variable naming
- No code duplication
- Proper error handling
- No exposed secrets
- Input validation
- Test coverage
- Performance considerations
- Time complexity analysis
- Third-party license compliance

Provide feedback by priority:
- Critical (must fix)
- Warnings (should fix)  
- Suggestions (consider improving)

Include specific examples for fixes.
```

### Creating and invoking subagents

Interactive creation via `/agents` opens the management interface. Select "Create New Agent" and choose project or user scope. Use "Generate with Claude" by describing desired functionality and letting Claude generate configuration, or "Manual configuration" to fill in fields yourself. Configure name, description, tool selection (or leave blank to inherit all), edit the system prompt, choose visual color, and save for immediate availability.

Automatic invocation happens when Claude selects and uses subagents based on task context, keywords in requests, description field matching, file types being worked on, or error messages encountered.

Explicit invocation uses natural language: "Use the code-reviewer subagent to check my recent changes" or "Have the security-auditor review this authentication code." Alternative @ mention syntax: `@agent-code-reviewer please check my recent changes` or `@agent-performance-engineer help optimize this database query`.

Recommended practice: Explicit invocation is deterministic and preferred for production flows where unexpected delegation should be avoided.

### Integration with Claude Code

The built-in Plan subagent activates automatically in plan mode to conduct codebase research, prevents infinite nesting (subagents cannot spawn other subagents), and returns findings to the main agent for plan creation.

Plugin agents provided by plugins appear in `/agents` alongside custom agents, can be invoked explicitly or automatically, work identically to user-defined agents, and reside in the plugin's `agents/` directory.

MCP integration means subagents access MCP tools when the `tools` field is omitted (inheriting all MCP tools from the main thread), can connect to external services like Slack, GitHub, Google Drive, and Asana, with authentication handled automatically by MCP, and support remote MCP servers as of June 2025.

### Practical workflows

Full-stack development pipelines use multiple specialized agents in sequence. The pm-spec agent writes working specifications and asks clarifying questions. The architect-review agent validates design and produces Architecture Decision Records. The implementer-tester agent implements code with tests. The security-reviewer agent scans for vulnerabilities. The test-runner agent executes tests and fixes failures.

Continuous testing in parallel uses Terminal 1 for interactive coding with `claude` and Terminal 2 for continuous tests in print mode: `claude -p --allowedTools "Bash(vendor/bin/pest:*)" "Read" "Grep" "Glob" "Edit" "Write" "Use the test-runner sub agent to run tests continuously and fix failures."`

### Limitations and best practices

Current limitations include no nested subagents (preventing infinite recursion), no plan mode in subagents (they execute immediately), no interactive thinking mode (difficult to monitor progress), sequential execution (tool calls don't run in parallel within a single REPL), initialization cost varying based on tool count, and latency from starting with clean slate each time.

Best practices include starting with Claude generation, following single responsibility principle (one clear goal per agent), using proactive keywords like "PROACTIVELY" or "MUST BE USED" in descriptions, practicing permission hygiene (only grant necessary tools), providing examples in system prompts, progressive tool expansion (start limited, add as validated), version controlling agents in `.claude/agents/`, and strategic model selection (Haiku for lightweight tasks at 3x cost savings, Sonnet/Opus for complex reasoning).

## Model Context Protocol (MCP) integration

MCP is Orion AI's open-source standard enabling Claude Code to connect to external tools, databases, and APIs through a universal protocol. Introduced in November 2024, MCP solves the "N×M problem" where every data source previously required custom implementation.

### MCP architecture

Claude Code functions as both an MCP Client (connecting to external servers) and an MCP Server (exposing its tools to other MCP clients). This dual role enables accessing issue trackers, querying databases, integrating design tools, connecting monitoring systems, and automating workflows.

The architecture consists of four components. The **MCP Host** (Claude Code terminal or VS Code extension) manages multiple clients, enforces security policies, coordinates AI integration, and aggregates context. **MCP Clients** maintain 1:1 isolated connections with servers, handle protocol negotiation, route messages bidirectionally, and maintain security boundaries. **MCP Servers** are lightweight programs exposing specific capabilities through tools, resources, and prompts. The **Base Protocol** built on JSON-RPC 2.0 defines message formats and communication patterns.

Three transport mechanisms support different use cases. **stdio** (standard input/output) enables local process communication with JSON-RPC via stdin/stdout, ideal for local tools with fastest performance. **HTTP + SSE** (Server-Sent Events) supports remote server communication with events via SSE, good for cloud services and multiple client connections. **HTTP POST** provides standard web API access with widest support for remote services.

### MCP capabilities

Three capability types extend Claude Code's functionality.

**Tools** are functions callable by Claude with user approval, like query_database, create_github_issue, or send_slack_message. They require approval for security.

**Resources** are file-like data accessible to Claude including API responses, file contents, and database records. Reference them using `@` syntax: `@server:protocol://resource/path`.

**Prompts** are pre-written templates for common tasks that become slash commands using format `/mcp__servername__promptname`. They accept arguments for dynamic execution.

Key features include capability negotiation where servers declare supported features during initialization, sampling that lets servers request LLM completions from clients for agentic workflows, elicitation for requesting additional information mid-operation using structured JSON schemas, OAuth 2.0 authentication with secure token storage and automatic refresh, and output management with configurable limits (default warning at 10,000 tokens, max 25,000 via `MAX_MCP_OUTPUT_TOKENS`).

### Setup and configuration

Remote HTTP servers use `claude mcp add --transport http [name] [url]`, for example `claude mcp add --transport http sentry https://mcp.sentry.dev/mcp`.

Remote SSE servers use `claude mcp add --transport sse [name] [url]`, for example `claude mcp add --transport sse asana https://mcp.asana.com/sse`.

Local stdio servers use `claude mcp add --transport stdio [name] [--env KEY=value] -- [command]`, for example:

```bash
claude mcp add --transport stdio github \
  --env GITHUB_PERSONAL_ACCESS_TOKEN=ghp_xxx \
  -- npx -y @modelcontextprotocol/server-github
```

Configuration scopes follow precedence: Local (private to you, project-specific, highest priority) > Project (shared via `.mcp.json`, version-controlled) > User (available across all projects, personal). The `.mcp.json` format defines server configurations with command, args, environment variables, and URLs with automatic `${VAR}` environment variable expansion.

Management commands include `claude mcp list` (show all servers), `claude mcp get [name]` (view specific config), `claude mcp remove [name]` (delete server), `claude mcp import --scope user` (import from Claude Desktop), and `claude mcp reset-project-choices` (reset approval choices).

### Usage patterns

The `/mcp` command opens interactive management for viewing connected servers, authenticating with OAuth 2.0, checking server health, and clearing authentication tokens.

Resource references use `@` autocomplete to show available resources, reference specific resources with `@server:protocol://resource/path`, and handle multiple resources in one prompt: `@github:repo://myorg/myrepo/README.md @slack:channel://general/recent-messages`.

Prompt execution uses slash commands listed with `/`, executed with `/mcp__servername__promptname`, and accepts arguments: `/mcp__github__create_issue title="Bug fix" priority="high"`.

### Popular MCP servers

Development and testing servers include Sentry (error monitoring), Socket (vulnerability scanning), Hugging Face (ML models), and Jam (bug reporting with session replay).

Project management integrations cover Asana (task management), Atlassian (Jira and Confluence), Linear (issue tracking), Notion (documentation), and Monday (project boards).

Databases and data sources include PostgreSQL (relational queries), Airtable (spreadsheet database), Daloopa (financial data), and HubSpot (CRM access).

Payments and commerce platforms cover Stripe, PayPal, Square, and Plaid (banking data).

Design and media tools include Figma (design files), Canva (templates), and InVideo (video generation).

Infrastructure and DevOps services include Netlify and Vercel (deployment), Stytch (authentication), AWS (cloud resources), and Cloudflare (CDN and security).

Communication platforms include Slack (team messaging), Gmail (email automation), and Intercom (customer support).

### Real-world workflows

Feature development from issue tracker: "Add the feature described in JIRA issue ENG-4521 and create a PR on GitHub" flows through JIRA MCP → GitHub MCP → Create PR.

Production debugging: "Check Sentry and Statsig for usage of feature ENG-4521" flows through Sentry MCP → Statsig MCP → Analysis.

User research: "Find 10 random users who used feature X from our Postgres database, then create Gmail drafts inviting them to feedback" flows through Postgres MCP → Gmail MCP → Draft emails.

Design integration: "Update our email template based on new Figma designs posted in Slack" flows through Slack MCP → Figma MCP → Template update.

### Enterprise features

Centralized MCP management uses managed configuration files at OS-specific locations. Access control implements allowlists (`allowedMcpServers` with `undefined` for no restrictions, `[]` for complete lockdown, or specific server lists) and denylists (`deniedMcpServers` with absolute precedence). Plugin-provided MCP servers bundle automatically with plugins, defined in `.mcp.json` at plugin root, and support all transports. Claude Code as MCP Server exposes its tools to other clients, configured in their settings with `"command": "claude", "args": ["mcp"]`.

## Context viewing features

Understanding what context Claude has access to is critical for effective collaboration and resource management. Claude Code 2.0 provides three primary mechanisms for inspecting context.

### /context command

Available since version 1.0.86, this command displays comprehensive token breakdown across system prompt, system tools, MCP tools, memory files (CLAUDE.md), custom agents, and messages. The output shows token usage for each category and percentage of context window remaining. This enables data-driven decisions about MCP server management and agent optimization. You can ask Claude to analyze the output and suggest optimizations. The accuracy can vary and there's no interactive toggle UI, but it provides the most detailed programmatic view of context consumption.

### /status command  

This interactive three-tab dashboard navigable with Tab key provides multiple views of system state. The Status tab shows version, session ID, account information, current model, IDE integration status, and configuration sources. The Config tab displays settings for auto-compact, checkpoints, theme, notifications, output style, editor mode, and model selection with current values. The Usage tab presents current session percentage, weekly usage percentage, and Opus usage tracking with visual progress bars. Limitations include interactive session only (no export), modal interface that blocks other work, static snapshot without history, and session scope without cross-session analysis.

### External tool: cccontext

This third-party CLI provides enhanced context monitoring capabilities. Real-time monitoring with `npx cccontext` shows live updates, `npx cccontext --list` displays available sessions, and `npx cccontext -session <num>` views specific session details. This is ideal for parallel development workflows using tmux, iTerm2, or other terminal multiplexers where you want to monitor context while working in another terminal.

## Additional major features

Beyond the core command, plugin, subagent, and MCP systems, Claude Code 2.0 includes numerous features that enhance development workflows.

### Checkpoint and rewind system

Automatic checkpointing saves both code state and conversation before each Claude edit, persists across session restarts, and requires no manual intervention. The `/rewind` command activates via Esc+Esc or `/rewind`, presenting three restoration options: conversation only (keep code changes), code only (keep conversation), or both (restore everything to prior state).

Use cases include experimental refactoring when trying different approaches, conversation branching to explore alternative solutions, and quick iteration testing multiple implementations. Important limitations: this is NOT a Git replacement and doesn't track bash command changes, has no checkpoint naming or cross-session branching, and only tracks Claude's file edits not manual changes.

### Terminal interface enhancements

Version 2.0 introduced prompt search with Ctrl+R for reverse history search like bash/zsh, with highlighted matches and cycling through results. The familiar UX enables fast command reusability but only supports exact match without filtering and has session-only scope.

Keyboard shortcuts include Esc (interrupt Claude), Esc+Esc (rewind or edit previous prompt), Shift+Tab (cycle modes: Default/Plan/YOLO), Tab (file path autocomplete after @), Ctrl+V (paste images, not Cmd+V), Up Arrow (navigate chat history), Ctrl+U (undo prompt input), Ctrl+Z (suspend Claude Code, resume with fg), and Ctrl+C (cancel current operation).

### VS Code extension

The beta extension provides native IDE integration with dedicated sidebar panel, real-time inline diffs showing changes as Claude makes them, accept/reject controls file by file for granular change management, and visual workflow without terminal switching. Feature parity with CLI is still developing with some commands like `/rewind` not yet available as of v2.0.1.

### File handling and navigation

The `@` reference syntax is the fastest way to add files to context: `@src/components/Header.tsx`. It supports Tab completion, multiple files in one prompt (`@file1.ts @file2.ts`), cross-monorepo references, MCP resources (`@server:protocol://resource/path`), wildcard-like behavior for partial matching, and case sensitivity.

The `--add-dir` flag or `/add-dir` command adds multiple directories without restarting: `claude --add-dir /path` or mid-session `/add-dir ../other-project`. This enables seamless workflow expansion across related codebases.

Agentic search avoids requiring codebase indexing by using Read, Grep, and List tools dynamically. Code stays local without external transmission, maps codebases in seconds, and discovers dependencies on-demand through intelligent exploration.

### Settings and configuration

The hierarchical system follows priority order: Enterprise Managed Settings (highest), User Settings (~/.claude/settings.json), Shared Project (.claude/settings.json), Local Project (.claude/settings.local.json), Command line flags (lowest).

Configuration methods include the `/config` command for visual interface, direct JSON editing for advanced control, and runtime flags for temporary overrides. Key configurable areas include model selection, permissions (allow/deny rules), MCP servers, environment variables, hooks, output styles, themes, and numerous operational parameters.

### Hooks system

Eight lifecycle events enable workflow customization. **Notification** handles authorization and confirmation alerts. **Stop** triggers on task completion and can add AI messages or text-to-speech. **SubagentStop** fires when background tasks complete. **PreToolUse** executes before tool execution and can block dangerous operations. **PostToolUse** runs after tool completion for logging or cleanup. **UserPromptSubmit** occurs before Claude processes input for validation or context injection. **PreCompact** fires before compaction for backup purposes. **SessionStart** handles session initialization including environment setup and CLAUDE_ENV_FILE access. **SessionEnd** enables cleanup but cannot block termination.

Control mechanisms use exit codes (0 for success, 2 to block), JSON responses for structured feedback, and matchers for pattern-based triggering. Use cases include auto-formatting after file writes, test enforcement after code changes, notifications for important events, security checks blocking dangerous operations, and comprehensive logging.

### Background tasks

Long-running processes execute without blocking through simple commands: "Run npm run dev in background" creates a bash_1 task. Check status with `/bashes`, view output with "Check bash_1 output", and kill with "Kill bash_1" or "Kill all background tasks". Tasks persist across sessions, with BashOutput returning only new output since last check. Use cases include dev servers, build watchers, Docker containers, database servers, and CI pipeline monitoring.

### Output styles

This revolutionary feature completely replaces the system prompt to transform Claude from coding assistant to any domain expert while maintaining all tools and capabilities. Built-in styles include Default (standard software engineering), Explanatory (educational with detailed insights), and Learning (mentorship mode with TODO markers for hands-on guidance).

Create custom styles with `/output-style:new I want an output style that acts as a business analyst...` or similar description. Storage in `~/.claude/output-styles/` makes styles portable, and switching happens instantly with `/output-style [name]`. Use cases span business analysis, UX research, performance coaching, content creation, specialized reviewers, and domain-specific assistants. The limitation is that base training patterns may occasionally override custom instructions for certain types of requests.

### CLAUDE.md project context

This project "constitution" file lives in the project root and provides Claude with project-specific context including overview and purpose, architecture and technology stack, coding standards and conventions, file organization patterns, common commands for development, typical workflows for features/bugs/reviews, testing approaches, security considerations, and team-specific practices.

Create it automatically with `/init` (Claude scans and generates starter file) or manually write it. Optimal size is 13KB typical, up to 25KB for large projects. Best practices include keeping it concise, referencing external docs rather than embedding them, providing alternatives instead of just prohibitions, and using it as a forcing function to simplify complex tooling. The file is a user message (not system prompt like `--append-system-prompt` or output styles).

### Permissions system

Three permission types control security. **Allow** auto-approves safe operations like Read, Grep, or specific npm commands. **Ask** prompts for approval (default for most operations). **Deny** blocks completely for sensitive files (.env, .ssh, secrets) or dangerous commands (rm -rf, curl to untrusted sources).

Permission modes include Default (prompt for each tool), Plan (create plan before execution), acceptEdits (auto-accept file edits), and bypassPermissions (dangerous, skips all prompts).

The experimental sandbox feature (`--sandbox` or `/sandbox`) provides filesystem and network isolation with 84% reduction in permission prompts and OS-level enforcement using Linux bubblewrap or macOS seatbelt. This is currently a research preview.

Security best practices include denying sensitive files (.env, .ssh, keys, secrets), blocking dangerous commands, using sandbox mode when available, testing configurations before production use, and regularly reviewing permission logs.

### Headless mode and CI/CD

Basic headless usage: `claude -p "prompt" --output-format stream-json` for automation. Key flags include `-p` (headless/print mode), `--output-format` (json, stream-json, markdown), `--allowedTools` (comma-separated list), `--permission-mode` (bypass, ask, acceptEdits), `--verbose` (detailed logging), `--max-turns` (limit agentic loops), `--continue` (resume last session), and `--resume <session-id>` (resume specific session).

Streaming input uses `--input-format stream-json` for JSONL input, enabling sophisticated piping and automation. Use cases include GitHub Actions for auto-labeling issues or PR reviews, pre-commit hooks for lint checks and quality validation, CI/CD for automated testing and code generation, and batch processing for file migrations or analysis.

Best practices include using verbose logging for debugging but disabling in production, handling exit codes (0 for success, non-zero for errors), parsing JSON output with jq or similar, ensuring stable network for MCP operations, and using prompt templates for consistency.

### Version control integration

Claude Code excels at Git operations with deep understanding of version control workflows. Git capabilities include history search ("What changed in v1.2.3?"), automatic commit message generation based on changes, complex operations like rebase, conflict resolution, cherry-pick, and patches, understanding shorthand like "pr" for pull requests, and branch management.

GitHub integration through `/install-github-app` enables automatic PR reviews. Issue management includes labeling, triaging, and analyzing issues. Review comments support one-shot resolution of reviewer feedback. Security features detect bugs and scan vulnerabilities.

### Session management

Sessions are stored in `~/.claude/projects/` as JSONL format. Management uses `claude` (start new session in current directory), `--resume <id>` (resume specific session), and `--continue` (resume last session from current directory). Parallel sessions support multiple terminals and git worktrees simultaneously. Session context includes working directory, loaded configuration, active MCP servers, permission choices, and model selection.

### Modes and workflows

Three chat modes cycle with Shift+Tab. **Default mode** suggests actions and waits for permission (safest). **Plan mode** creates detailed plan first for review before execution. **YOLO mode** auto-accepts edits while still asking for bash commands (fastest).

Workflow patterns include Explore-Plan-Act (Read codebase → Plan changes → Execute implementation), Multi-Claude (one writes code, another reviews), Checklist Method (maintain Markdown task list), and Document & Clear (save progress, clear context, continue fresh).

### Additional capabilities

Image support includes screenshot paste (Ctrl+V), drag-and-drop to terminal, and file path references for existing images. Web integration uses WebFetch for accessing URLs and external documentation. Model selection via `/model` chooses between Sonnet 4.5 (default), Opus (most capable), or Haiku (fastest, cheapest). Context management tools include `/clear` (wipe history), `/compact` (summarize), `/context` (view breakdown), and `/status` (dashboard). Interrupt and correction use Esc (interrupt), Esc+Esc (rewind), or natural language like "undo that" or "no, let's try a different approach". Parallel worktrees support multiple git worktrees with separate Claude sessions. Codebase understanding enables asking any question with automatic intelligent search.

## Best practices and workflows

Effective use of Claude Code 2.0 requires understanding patterns that maximize productivity while maintaining code quality and security.

### Test-driven development

Start by asking Claude to write tests based on expected behavior, run tests to see them fail, request implementation to pass tests, and iterate with refinement. This ensures code meets requirements and maintains testability.

### Feature development workflow

Ask Claude to read relevant files and understand context, request a detailed plan before implementation, ask for implementation with explicit verification steps, request tests and validation, and have Claude commit and create PR with descriptive messages. Planning first prevents wasted effort from incorrect assumptions.

### Debugging workflow  

Provide error message and relevant context, ask Claude to search codebase for root cause, request fix with explanation, verify fix with tests, and use `/rewind` if the fix doesn't work. Systematic debugging finds root causes rather than applying band-aids.

### Code review workflow

Use `/review @./src/components/NewFeature.tsx` and ask Claude to check code quality, security vulnerabilities, performance issues, adherence to project standards, and test coverage. Automated review catches issues before human review.

### Token management

Use `/clear` frequently when switching tasks, `/compact` to summarize long sessions, extract repeated workflows to custom slash commands, and keep CLAUDE.md concise and focused. Proactive token management prevents hitting limits and controls costs.

### Safety practices

Always use version control (git), review Claude's changes before accepting, test thoroughly after AI changes, create checkpoints before risky refactors with explicit `/rewind` readiness, never use `--dangerously-skip-permissions` in production, never trust AI output without verification, never skip testing after major changes, and never ignore security recommendations.

## Conclusion

Claude Code 2.0 represents a comprehensive autonomous development platform that extends far beyond code completion. The extensive slash command system provides both built-in and customizable workflows. The plugin marketplace enables sharing and discovering community-built functionality. Subagents enable specialized delegation with parallel execution. MCP integration connects Claude to external tools and data sources across the development ecosystem. Context viewing features provide visibility into resource usage. Additional features including checkpoints, hooks, background tasks, output styles, permissions, and headless mode create a complete development environment.

Success with Claude Code comes from understanding its breadth of capabilities, strategically using the right features for each task, maintaining clean context and permissions hygiene, leveraging community plugins and agents, integrating with existing tools via MCP, and always treating AI assistance as a collaborator requiring verification. This tool transforms software development from individual coding to orchestrating an AI-assisted team, with you as the architect and reviewer ensuring quality while Claude handles implementation details, exploration, and automation at unprecedented scale and speed.