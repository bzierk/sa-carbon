# SA-Carbon: Star Atlas Account and Instruction Deserialiation

An example implementation of a real-time monitoring and decoding tool for Solana accounts and instructions using the Carbon indexing framework. This project provides a comprehensive decoder for Star Atlas program data and a configurable pipeline for monitoring blockchain activity.

## Overview

SA-Carbon consists of two main components:

1. **Sage Decoder** (`decoders/sage-decoder/`): A comprehensive decoder library for Star Atlas program accounts and instructions.
2. **Demo Application** (`demo/`): An example monitoring application that processes Star Atlas data in real-time

The demo application supports multiple data sources and can be configured to monitor different aspects of the Star Atlas blockchain program.

## Features

- **Multiple Data Sources**: Support for RPC Transaction Crawler, Yellowstone gRPC Geyser, and RPC Program Subscribe
- **Real-time Processing**: Live monitoring of Star Atlas accounts and instructions
- **Comprehensive Decoding**: Currently supports all account and instruction types for the Holosim deployment of Star Atlas's SAGE product
- **Configurable Logging**: Environment-based configuration for different log levels and data sources
- **Flexible Architecture**: Easy to extend with new processors and data sources

## Installation

### Prerequisites

- Rust 1.70+ (2024 edition)
- Access to a Solana RPC endpoint
- Optional: Yellowstone Geyser endpoint for high-performance streaming



## Configuration

The application is configured via environment variables. Create a `.env` file in the project root:

### Required Environment Variables

```env
# Data source selection (required)
DATASOURCE_TYPE=transaction_crawler  # Options: transaction_crawler, geyser, program_subscribe

# Logging configuration
RUST_LOG=info  # Options: trace, debug, info, warn, error
```

### Data Source Specific Variables

#### RPC Program Subscribe (Default)

```env
DATASOURCE_TYPE=subscribe
RPC_WS_URL=wss://api.mainnet-beta.solana.com
```

#### Transaction Crawler

```env
DATASOURCE_TYPE=crawler
RPC_URL=https://api.mainnet-beta.solana.com
```

#### Yellowstone Geyser

```env
DATASOURCE_TYPE=geyser
GEYSER_URL=https://your-geyser-endpoint.com
X_TOKEN=your_authentication_token  # Optional, if required by your provider
```

### Complete Example `.env` File

```env
# Data source configuration
DATASOURCE_TYPE=transaction_crawler
RPC_URL=https://api.mainnet-beta.solana.com

# Alternative Geyser configuration (uncomment to use)
# DATASOURCE_TYPE=geyser
# GEYSER_URL=https://your-geyser-endpoint.com
# X_TOKEN=your_token_if_required

# Alternative WebSocket configuration (uncomment to use)
# DATASOURCE_TYPE=program_subscribe
# RPC_WS_URL=wss://api.mainnet-beta.solana.com

# Logging configuration
RUST_LOG=info
```

## Usage

### Basic Usage

1. Create your `.env` file with the required configuration
2. Run the application:

```bash
cargo run --bin demo
```

### Development Mode

For development with detailed logging:

```bash
RUST_LOG=debug cargo run --bin demo
```

### Production Mode

For production with optimized performance:

```bash
cargo run --release --bin demo
```

## Data Source Options

### 1. Transaction Crawler

- **Use Case**: Crawls historical successful transactions for a specific address in reverse chronological order using Solana JSON RPC
- **Configuration**: Only requires `RPC_URL`

```env
DATASOURCE_TYPE=transaction_crawler
RPC_URL=https://api.mainnet-beta.solana.com
```

### 2. Yellowstone Geyser

- **Use Case**: Subscribes to a Yellowstone gRPC Geyser plugin enhanced full node to stream account and transaction updates
- **Configuration**: Requires `GEYSER_URL`, optionally `X_TOKEN`

```env
DATASOURCE_TYPE=geyser
GEYSER_URL=https://your-geyser-endpoint.com
X_TOKEN=your_authentication_token
```

### 3. RPC Program Subscribe

- **Use Case**: Uses programSubscribe with Solana WS JSON RPC to listen to real-time on-chain account updates
- **Configuration**: Requires `RPC_WS_URL`

```env
DATASOURCE_TYPE=program_subscribe
RPC_WS_URL=wss://api.mainnet-beta.solana.com
```

## Output and Logging

The application outputs structured logs for different account types:

- **SAGE accounts**: Logged at `DEBUG` level
- **Unknown accounts**: Logged at `WARN` level
- **Instructions**: Logged at `INFO` level with nested instruction details

Example output:

```
Processing update
Fleet: Fleet {
    version: 0,
    game_id: "...",
    owner_profile: "...",
    fleet_state: Idle(Idle { sector: [x, y] }),
    // ... additional fields
}
```

## Development

### Project Structure

```
sa-carbon/
├── decoders/
│   └── sage-decoder/          # Core decoder library
│       ├── src/
│       │   ├── accounts/      # Account type definitions
│       │   ├── instructions/  # Instruction decoders
│       │   └── types/         # Common type definitions
│       └── Cargo.toml
├── demo/                      # Demo monitoring application
│   ├── src/
│   │   └── main.rs           # Main application logic
│   └── Cargo.toml
├── .env                      # Environment configuration
└── README.md
```

### Adding New Account Types

1. Add the account struct to `decoders/sage-decoder/src/accounts/`
2. Implement the necessary traits (`BorshDeserialize`, `CarbonDeserialize`)
3. Add the variant to the `SageAccount` enum
4. Update the processor in `demo/src/main.rs`

### Extending Processors

The application uses a processor pattern. To add custom processing logic:

1. Implement the `Processor` trait
2. Add your processor to the pipeline in `main.rs`
3. Configure any additional data sources as needed

