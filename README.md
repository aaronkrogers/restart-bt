# restart-bt

A lightweight web service that enables restarting the Bluetooth service via NFC tag scanning. This application runs as a local HTTP server and validates NFC tag IDs before executing the restart command.

## Purpose

`restart-bt` provides a convenient way to restart the Linux Bluetooth service (`systemctl restart bluetooth`) when an authorized NFC tag is scanned. This is useful for automating Bluetooth troubleshooting and recovery workflows without requiring direct command-line access.

## Features

- **NFC Tag Validation**: Only authorized NFC tag IDs can trigger Bluetooth restarts
- **Web-based API**: Simple HTTP POST endpoint for initiating restarts
- **Configurable Binding**: Control which network interface the service binds to
- **Customizable Port**: Configure the listening port (defaults to 8989)
- **Version Endpoint**: Query the application version via HTTP

## Installation

### Prerequisites

- Rust (1.70 or later)
- Linux system with systemd and Bluetooth service
- sudo access (required to restart the Bluetooth service)

### Building

```bash
cargo build --release
```

The compiled binary will be available at `target/release/restart-bt`.

## Usage

### Starting the Server

```bash
# Run with default settings (localhost:8989)
./restart-bt

# Bind to a specific interface and port
./restart-bt --bind 192.168.1.100 --port 9000

# Bind to all interfaces (0.0.0.0) on port 8989
./restart-bt --bind 0.0.0.0
```

### Command-line Options

- `--bind <IP_ADDRESS>`: IP address to bind to (default: 127.0.0.1)
- `--port <PORT>` or `-p <PORT>`: Port to listen on (default: 8989)

### Environment Variables

- `ALLOWED_TAGS`: Comma-separated or whitespace-separated list of authorized NFC tag IDs (uppercase or lowercase). **Required** - if not set, no tags will be accepted.

### Example

```bash
export ALLOWED_TAGS="AA:BB:CC:DD:EE:FF,01:02:03:04:05"
./restart-bt --bind 0.0.0.0 --port 8989
```

## API Endpoints

### GET `/version`

Returns the application version.

**Request:**
```
GET /version HTTP/1.1
```

**Response:**
```
HTTP/1.1 200 OK
0.1.1
```

### POST `/restart-bluetooth`

Restarts the Bluetooth service if the provided NFC tag ID is authorized.

**Request:**
```
POST /restart-bluetooth HTTP/1.1
Content-Type: application/x-www-form-urlencoded

tagid=AA:BB:CC:DD:EE:FF
```

**Success Response:**
```
HTTP/1.1 200 OK
```

**Error Response:**
```
HTTP/1.1 500 Internal Server Error
invalid tagid
```

## Configuration

### Systemd Service (Optional)

To run `restart-bt` as a systemd service:

```ini
[Unit]
Description=Bluetooth Restart Service
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/restart-bt --bind 0.0.0.0 --port 8989
Environment=ALLOWED_TAGS="AA:BB:CC:DD:EE:FF,01:02:03:04:05"
Restart=on-failure
RestartSec=10
# Run as a user having sudo access to call `systemctl restart bluetooth` without a password
User=http

[Install]
WantedBy=multi-user.target
```

## Security Considerations

- The service requires **sudo privileges** to execute `systemctl restart bluetooth`
- Only authorized NFC tag IDs (configured via `ALLOWED_TAGS`) can trigger restarts
- By default, the service binds to localhost only; use `--bind` with caution when exposing to a network
- Ensure the `ALLOWED_TAGS` environment variable is kept secure
- Tag IDs are case-insensitive and automatically converted to uppercase

## Requirements

- The application must have sudo access to execute `systemctl restart bluetooth`
- The Linux system must have the Bluetooth service installed and configured with systemd

## License

See LICENSE file for details.
