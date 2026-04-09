# ai-switch

Manage AI provider configurations for various coding tools.

## Installation

```bash
cargo install --path .
```

## Quick Start

```bash
# 1. Initialize
ai-switch init

# 2. Add provider (will auto-prompt for Claude model presets)
ai-switch provider add anthropic

# 3. Add target
ai-switch target add claude --target-type claude-code

# 4. Apply (use default model)
ai-switch use anthropic --target claude

# 5. Switch model
ai-switch use anthropic --target claude --model haiku
```

## Commands

### Provider Management

```bash
# Add provider (interactive - will ask about API Key, Base URL, model presets)
ai-switch provider add <name>

# List providers
ai-switch provider list

# Remove provider
ai-switch provider remove <name>
```

### Model Profiles

Adding a provider will prompt you to add Claude model presets (haiku/sonnet/opus) with default model IDs:

```bash
$ ai-switch provider add anthropic
API Key: sk-ant-xxxxx
Base URL (optional):
Add Claude model presets (haiku/sonnet/opus)? [y/N]: y
Haiku model ID [claude-haiku-4-20250514]:
Sonnet model ID [claude-sonnet-4-20250514]:
Opus model ID [claude-opus-4-20250514]:

Added model profiles: haiku, sonnet, opus (default: sonnet)
```

Manual model profile management:

```bash
# Add a model profile
ai-switch provider model add <provider> <name> --model <model_id> [--default]

# List model profiles
ai-switch provider model list <provider>

# Remove a model profile
ai-switch provider model remove <provider> <name>

# Set default model
ai-switch provider model set-default <provider> <name>
```

### Target Management

```bash
# Add target
ai-switch target add <name> --target-type <type>

# List targets
ai-switch target list

# Remove target
ai-switch target remove <name>
```

### Apply & Status

```bash
# Apply provider to target (use default model)
ai-switch use <provider> --target <target>

# Apply specific model
ai-switch use <provider> --target <target> --model <model_name>

# Show current config
ai-switch current

# Show full status
ai-switch status
```

## Supported Providers

- OpenAI
- Anthropic
- Zhipu (智谱 AI)
- Google Gemini

## Supported Targets

- Claude Code
- Cursor (planned)
- aider (planned)
