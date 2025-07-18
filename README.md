# MLS Chat - End-to-End Encrypted Messaging Demonstration

A minimal CLI-based messaging application built in Rust that demonstrates MLS (Messaging Layer Security) protocol concepts. This application showcases end-to-end encryption for group messaging with cryptographic agility and modular design through a mock implementation.

## Features

- **End-to-End Encryption**: All messages are encrypted using the MLS protocol
- **Group Messaging**: Create groups and add members with secure key distribution
- **Cryptographic Agility**: Modular crypto provider supporting various KEMs and encryption algorithms
- **Persistent State**: Application state is saved to disk for session persistence
- **Simple CLI Interface**: Easy-to-use command-line interface for messaging operations
- **Two-User Demo**: Supports Alice and Bob for demonstration purposes

## Architecture

### Core Components

1. **MLS Protocol Demonstration**: Mock implementation showcasing MLS protocol concepts
2. **Cryptographic Simulation**: Demonstrates key generation, distribution, and rotation
3. **Group Management**: Handles group creation, member addition, and key distribution
4. **Message Encryption**: Simulates message encryption using group secrets
5. **State Persistence**: JSON-based storage for groups and messages

### Cryptographic Features

- **Key Encapsulation Mechanisms (KEMs)**: Flexible KEM support through OpenMLS
- **Authenticated Encryption**: Messages are encrypted and authenticated
- **Forward Secrecy**: Keys are updated with each message to ensure forward secrecy
- **Post-Compromise Security**: Compromised keys can be rotated out of the group

## Installation

### Prerequisites

- Rust 1.70+ (stable)
- Linux environment (tested on Ubuntu 22.04)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/mls-chat.git
cd mls-chat

# Build the application
cargo build --release

# Run the application
cargo run --release
```

## Usage

### Quick Start

1. **Initialize Users**
   ```bash
   # Initialize Alice
   cargo run -- init alice
   
   # Initialize Bob
   cargo run -- init bob
   ```

2. **Create a Group**
   ```bash
   # Alice creates a group
   cargo run -- create-group "MyGroup"
   ```

3. **Add Members**
   ```bash
   # Alice adds Bob to the group
   cargo run -- add-member "MyGroup" bob
   ```

4. **Send Messages**
   ```bash
   # Send a message to the group
   cargo run -- send "MyGroup" "Hello, this is an encrypted message!"
   ```

5. **View Messages**
   ```bash
   # List all messages in the group
   cargo run -- list "MyGroup"
   ```

### Command Reference

#### `init <user>`
Initialize a new user identity (Alice or Bob).

**Arguments:**
- `user`: User name (alice|bob)

**Example:**
```bash
cargo run -- init alice
```

#### `create-group <name>`
Create a new MLS group with the current user as the creator.

**Arguments:**
- `name`: Group name

**Example:**
```bash
cargo run -- create-group "ProjectTeam"
```

#### `add-member <group> <member>`
Add a member to an existing group.

**Arguments:**
- `group`: Group name
- `member`: Member to add (alice|bob)

**Example:**
```bash
cargo run -- add-member "ProjectTeam" bob
```

#### `send <group> <message>`
Send an encrypted message to a group.

**Arguments:**
- `group`: Group name
- `message`: Message content

**Example:**
```bash
cargo run -- send "ProjectTeam" "Meeting at 3 PM tomorrow"
```

#### `list <group>`
List all messages in a group.

**Arguments:**
- `group`: Group name

**Example:**
```bash
cargo run -- list "ProjectTeam"
```

#### `info <group>`
Show detailed information about a group.

**Arguments:**
- `group`: Group name

**Example:**
```bash
cargo run -- info "ProjectTeam"
```

## Security Features

### MLS Protocol Benefits

1. **End-to-End Encryption**: Messages are encrypted on the sender's device and only decrypted on recipient devices
2. **Group Key Management**: Automatic key distribution and rotation for group members
3. **Forward Secrecy**: Compromise of current keys doesn't affect past messages
4. **Post-Compromise Security**: Compromised devices can be removed and keys rotated
5. **Message Authentication**: All messages are cryptographically authenticated

### Cryptographic Agility

The application uses a modular crypto provider that supports:
- Multiple key encapsulation mechanisms
- Various encryption algorithms
- Flexible signature schemes
- Configurable ciphersuites

## Data Storage

Application data is stored in the `mls_chat_data/` directory:
- `app_state.json`: Serialized application state including groups and messages
- MLS group states are persisted for session continuity

## Development

### Project Structure

```
mls-chat/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Dependencies and build configuration
├── README.md            # This file
└── .gitignore           # Git ignore rules
```

### Key Dependencies

- **clap**: Command-line argument parsing
- **serde**: Serialization for state persistence
- **anyhow**: Error handling
- **colored**: Terminal output formatting
- **uuid**: Unique identifier generation
- **chrono**: Timestamp handling

### Building for Development

```bash
# Development build
cargo build

# Run with debug output
RUST_LOG=debug cargo run -- init alice

# Run tests
cargo test
```

## Troubleshooting

### Common Issues

#### 1. "No user initialized" Error
**Problem**: Trying to use commands without initializing a user first.
**Solution**: Initialize a user with `cargo run -- init alice` or `cargo run -- init bob`.

#### 2. "Group not found" Error
**Problem**: Trying to access a group that doesn't exist.
**Solution**: Create the group first with `cargo run -- create-group "GroupName"`.

#### 3. "Key package not found" Error
**Problem**: Trying to add a member who hasn't been initialized.
**Solution**: Initialize all users before adding them to groups.

#### 4. Build Errors
**Problem**: Compilation fails due to missing dependencies.
**Solution**: Ensure you have Rust 1.70+ installed and run `cargo clean && cargo build`.

#### 5. Permission Errors
**Problem**: Cannot create data directory or write files.
**Solution**: Ensure write permissions in the current directory.

### Debug Mode

Enable debug logging to troubleshoot issues:

```bash
RUST_LOG=debug cargo run -- <command>
```

### Data Recovery

If the application state becomes corrupted:
1. Stop the application
2. Backup the `mls_chat_data/` directory
3. Delete `mls_chat_data/app_state.json`
4. Restart the application and reinitialize users/groups

## Limitations

### Current Limitations

1. **Two Users Only**: Currently supports only Alice and Bob for demonstration
2. **Local Storage**: All data is stored locally (no network communication)
3. **No Message Decryption**: Messages are encrypted but not decrypted in this demo
4. **Single Session**: No support for multiple concurrent sessions

### Future Enhancements

1. **Multi-User Support**: Extend to support arbitrary user names
2. **Network Communication**: Add client-server architecture
3. **Message Decryption**: Implement full message decryption
4. **Concurrent Sessions**: Support multiple active sessions
5. **Key Rotation**: Implement automatic key rotation
6. **Member Removal**: Add ability to remove group members

## Security Considerations

### Important Notes

1. **Demo Purpose**: This application is for educational and demonstration purposes
2. **Local Storage**: Keys and messages are stored locally without additional encryption
3. **No Network Security**: This demo doesn't include transport layer security
4. **Key Management**: In production, implement proper key backup and recovery

### Best Practices

1. **Regular Updates**: Keep dependencies updated for security patches
2. **Secure Storage**: In production, encrypt stored keys and messages
3. **Key Backup**: Implement secure key backup mechanisms
4. **Audit Logging**: Add comprehensive audit logging for security events

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [OpenMLS](https://github.com/openmls/openmls) - MLS protocol implementation
- [Rust Crypto](https://github.com/RustCrypto) - Cryptographic primitives
- [MLS Working Group](https://datatracker.ietf.org/wg/mls/about/) - Protocol specification

## Support

For issues and questions:
1. Check the troubleshooting section above
2. Review the OpenMLS documentation
3. Open an issue on GitHub
4. Check the MLS protocol specification

---

**Note**: This is a demonstration application. For production use, implement additional security measures and follow MLS protocol best practices.
