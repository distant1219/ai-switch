# ai-switch

Manage AI provider configurations for various coding tools.

## Installation

```bash
cargo install --path .
```

## Usage

### Basic Setup

```bash
# Initialize configuration
ai-switch init

# Add a provider
ai-switch provider add openai-work

# Add a target tool
ai-switch target add claude --target-type claude-code

# Apply configuration
ai-switch use openai-work --target claude

# Check status
ai-switch status
```

### Multi-Model Profiles

For providers supporting multiple models (like Anthropic with haiku/sonnet/opus):

```bash
# Add a provider with multiple model profiles
ai-switch provider add anthropic
ai-switch provider model add anthropic haiku --model claude-haiku-4-20250514 --default
ai-switch provider model add anthropic sonnet --model claude-sonnet-4-20250514
ai-switch provider model add anthropic opus --model claude-opus-4-20250514

# List model profiles
ai-switch provider model list anthropic

# Set default model profile
ai-switch provider model set-default anthropic sonnet

# Apply specific model to target
ai-switch use anthropic --target claude --model sonnet
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

## Example Configuration

```toml
[providers.claude-work]
api_key = "sk-ant-..."
base_url = "https://api.anthropic.com"
default_profile = "sonnet"

[providers.claude-work.models.haiku]
model = "claude-haiku-4-20250514"

[providers.claude-work.models.sonnet]
model = "claude-sonnet-4-20250514"

[providers.claude-work.models.opus]
model = "claude-opus-4-20250514"

[targets.claude]
type = "claude-code"
config_path = "~/.claude/settings.json"

[current]
claude = "claude-work"
```
