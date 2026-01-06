# Raushan Explorer

A production-grade, terminal-based Solana Blockchain Explorer written in Rust.

`raushan` provides deep introspection capabilities for the Solana blockchain directly from your CLI, eliminating the need for web-based explorers for development and debugging tasks. It supports Devnet, Testnet, and Mainnet-Beta.

## Installation

### Prerequisites

- Rust (1.75+)
- OpenSSL (libssl-dev)

### Install via Cargo

```bash
cargo install --path .
```

### Build from Source

```bash
git clone https://github.com/your-repo/raushan-explorer
cd raushan-explorer
cargo build --release
sudo cp target/release/raushan /usr/local/bin/
```

## Usage

The CLI is organized into command groups for intuitive access.

```bash
raushan <COMMAND> [OPTIONS]
```

### Common Commands

| Command          | Description                                             |
| ---------------- | ------------------------------------------------------- |
| `network-status` | Display current Epoch, Slot,Block Height, and progress. |
| `network-tps`    | Real-time Transactions Per Second tracker.              |
| `account-info`   | Detailed account balance, owner, and execution status.  |
| `tx-info`        | breakdown of a transaction including fees and logs.     |
| `cluster-info`   | View current cluster version and feature set.           |
| `validator-list` | List active validators ordered by stake.                |

### Configuration

Set your target cluster (persists across sessions):

```bash
raushan cluster-set devnet
raushan cluster-set mainnet
```

### Examples

**Check Network Health**

```bash
raushan network-status
```

**Inspect a Transaction**

```bash
raushan tx-info 5Sig...
```

**View Account details**

```bash
raushan account-info <ADDRESS>
```

**List Tokens**

```bash
raushan account-tokens <ADDRESS>
```

## License

MIT License
