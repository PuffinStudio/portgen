# Portgen

A Rust CLI tool that generates unique port numbers for your projects based on the project name. Avoids conflicts with common services like databases, web servers, and development tools.

## Features

- 🎯 **Deterministic**: Same project name always generates the same ports
- 🚫 **Smart Filtering**: Automatically avoids reserved ports (databases, web servers, etc.)
- 🔍 **Availability Check**: Tests if generated ports are currently in use
- ⚡ **Fast**: Written in Rust for optimal performance
- 🎨 **Beautiful Output**: Colored, emoji-enhanced terminal output

## Installation

### Using Cargo (Recommended)

Install directly from GitHub:

```bash
cargo install --git https://github.com/PuffinStudio/portgen.git
```

This will install `portgen` to `~/.cargo/bin/`. Make sure this directory is in your PATH.

### From Source

```bash
git clone https://github.com/PuffinStudio/portgen.git
cd portgen
cargo build --release
```

The binary will be at `target/release/portgen`.

### Prerequisites

- [Rust](https://rust-lang.org/) 1.70 or later

## Usage

### Basic Usage

Generate 2 ports for your project (default):

```bash
portgen my_project
```

### Generate Multiple Ports

```bash
# Generate 4 ports
portgen my_project --count 4

# Or use short flag
portgen my_project -c 4
```

### Use Current Directory Name

If no project name is provided, uses the current directory name:

```bash
cd my-awesome-app
portgen
# Output: Project name: my-awesome-app
```

### List Reserved Ports

See all ports that are avoided:

```bash
portgen --list-reserved
# or
portgen -l
```

### Help

```bash
portgen --help
```

## How It Works

1. **Hash Generation**: Converts your project name to a hash by summing ASCII values
2. **Port Calculation**: Maps hash to port range 1024-65535 (avoiding privileged ports < 1024)
3. **Reserved Filtering**: Skips known service ports (MySQL, PostgreSQL, Redis, etc.)
4. **Availability Testing**: Binds to port to verify it's not currently in use

### Reserved Ports

The tool automatically avoids 80+ common ports including:

- **Databases**: MySQL (3306), PostgreSQL (5432), Redis (6379), MongoDB (27017), Elasticsearch (9200)
- **Web Dev**: React/Vue (3000), Vite (5173), Django/Flask (8000), Angular (4200)
- **Services**: HTTP (80), HTTPS (443), SSH (22), SMTP (25), FTP (21)
- **Monitoring**: Prometheus (9090), Grafana (3000), Kibana (5601)
- **Message Queue**: Kafka (9092), RabbitMQ (5672), NATS (4222)

## CLI Options

```
Usage: portgen [OPTIONS] [PROJECT_NAME]

Arguments:
  [PROJECT_NAME]  Project name to generate ports for (defaults to current directory name)

Options:
  -c, --count <COUNT>  Number of ports to generate [default: 2]
  -l, --list-reserved  Show all reserved ports
  -h, --help           Print help
  -V, --version        Print version
```

## Examples

### Example 1: Web Application

```bash
$ portgen webapp
💡 No project name provided, using current directory name as default
🔧 Project name: webapp
📝 Generating port numbers based on project name...

🔢 Suggested port numbers:
   Port 1: 21543
   Port 2: 21556

🔍 Checking port availability...
✅ Port 21543 is available
✅ Port 21556 is available

📋 Summary:
✅ Available ports:
   - 21543
   - 21556

🎉 All ports are available!
```

### Example 2: API with Multiple Services

```bash
$ portgen api-service --count 3
🔧 Project name: api-service
📝 Generating port numbers based on project name...

🔢 Suggested port numbers:
   Port 1: 42356
   Port 2: 42369
   Port 3: 42382

🔍 Checking port availability...
✅ Port 42356 is available
✅ Port 42369 is available
✅ Port 42382 is available

📋 Summary:
✅ Available ports:
   - 42356
   - 42369
   - 42382

🎉 All ports are available!
```

### Example 3: Port in Use

```bash
$ portgen myapp
🔧 Project name: myapp
📝 Generating port numbers based on project name...

🔢 Suggested port numbers:
   Port 1: 18432
   Port 2: 18445

🔍 Checking port availability...
⚠️  Port 18432 is already in use
✅ Port 18445 is available

📋 Summary:
✅ Available ports:
   - 18445

⚠️  Only 1 port(s) available, requested 2
```

## Port Ranges

- **Valid Range**: 1024-65535 (avoids privileged ports 1-1023)
- **Algorithm**: Deterministic hash-based generation
- **Collision Handling**: Auto-increments to next available port

## Development

### Running Tests

```bash
cargo test
```

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

## Why Portgen?

When working on multiple projects, it's easy to forget which ports you've used:

- ❌ **Manual tracking**: Spreadsheets or notes get outdated
- ❌ **Random ports**: Hard to remember, no consistency across team
- ❌ **Port conflicts**: "Address already in use" errors

✅ **Portgen solves this**: Same project name → Same ports → Every time → Everywhere

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI parsing with [clap](https://github.com/clap-rs/clap)
