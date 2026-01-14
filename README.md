# Gity

**Gity** is an AI-powered CLI tool that automatically generates conventional git commit messages using Google's Gemini API.

## Features

- 🤖 **AI-Generated Messages**: Generates concise, descriptive commit messages based on your staged changes.
- 📝 **Conventional Commits**: Follows the [Conventional Commits](https://www.conventionalcommits.org/) specification.
- 🔄 **Interactive Mode**: Review the generated message, edit it in your favorite editor, or cancel before committing.
- 🚀 **Fast**: Uses the latest Gemini Flash models for quick responses.

## Installation

### From Source

Clone the repository and install using Cargo:

```bash
git clone https://github.com/hoshino-dev/gity.git
cd gity
cargo install --path .
```

This will install the `gity` binary to your Cargo bin directory (usually `~/.cargo/bin`).

### Uninstallation

To remove the tool:

```bash
cargo uninstall gity
```

## Configuration

You need a Google Gemini API key to use Gity. You can get one from [Google AI Studio](https://aistudio.google.com/).

> [!WARNING]
> **Data Privacy Notice**: If you use the free tier of the Gemini API, your input data (including staged code changes) may be used by Google for model training. It is recommended to use the paid tier or avoid using this tool for sensitive private repositories if ensuring data privacy is critical.

### Quick Setup

Run the following command to set your API key:

```bash
gity config --api-key "your_api_key_here"
```

This ensures the key is saved persistently.

### Environment Variables (Optional)

You can also set the key using environment variables (highest priority):

```bash
export GITY_GEMINI_API_KEY="your_api_key_here"
```

Or via a `.env` file in your project root:

```ini
GITY_GEMINI_API_KEY=your_api_key_here
```

## Usage

1. **Stage your changes**:
   ```bash
   git add .
   ```

2. **Run Gity**:
   ```bash
   gity
   ```

3. **Interact**:
   The tool will analyze your staged changes and propose a commit message. You will see a menu:
   - **Commit**: Accept the message and commit immediately.
   - **Edit**: Open the message in your default editor (configured via `$EDITOR` or `$VISUAL`) to make changes.
   - **Cancel**: Abort the commit.

## Development

To build and run locally:

```bash
# Install dependencies
cargo build

# Run
cargo run
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
