# ai-switch

Manage AI provider configurations for various coding tools.

## Installation

```bash
cargo install --path .
```

## Usage

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

## Supported Providers

- OpenAI
- Anthropic
- Zhipu (智谱 AI)
- Google Gemini

## Supported Targets

- Claude Code
- Cursor (planned)
- aider (planned)
