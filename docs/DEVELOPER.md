# Developer Documentation

## Architecture Overview

The MLS Chat application is built with a modular architecture that separates concerns and provides cryptographic agility. The application consists of several key components:

### Core Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    CLI Interface Layer                      │
├─────────────────────────────────────────────────────────────┤
│                   Application Logic Layer                   │
├─────────────────────────────────────────────────────────────┤
│                    MLS Protocol Layer                       │
├─────────────────────────────────────────────────────────────┤
│                  Cryptographic Provider                     │
└─────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

1. **CLI Interface Layer**: Handles command parsing and user interaction
2. **Application Logic Layer**: Manages groups, messages, and application state
3. **MLS Protocol Layer**: Implements the Messaging Layer Security protocol
4. **Cryptographic Provider**: Provides cryptographic primitives and operations

## Code Structure

### Main Application (`src/main.rs`)

The main application file contains all the core functionality:

#### Key Structs

```rust
struct MlsChatApp {
    crypto: OpenMlsRustCrypto,           // Cryptographic provider
    current_user: Option<UserName>,      // Current active user
    groups: HashMap<String, ChatGroup>,  // All groups
    key_packages: HashMap<String, KeyPackageBundle>, // User key packages
    data_dir: String,                    // Data storage directory
}
```

#### Core Methods

- `init_user()`: Initialize user identity and generate key packages
- `create_group()`: Create new MLS group with cryptographic setup
- `add_member()`: Add member to group with secure key distribution
- `send_message()`: Encrypt and send message using MLS protocol
- `list_messages()`: Display group messages
- `show_group_info()`: Display group metadata

### Data Models

#### ChatMessage
```rust
struct ChatMessage {
    id: String,                    // Unique message identifier
    sender: String,                // Sender username
    content: String,               // Message content
    timestamp: DateTime<Utc>,      // Message timestamp
    group_id: String,              // Associated group ID
}
```

#### ChatGroup
```rust
struct ChatGroup {
    name: String,                  // Group name
    group_id: String,              // MLS group identifier
    members: Vec<String>,          // Group member list
    messages: Vec<ChatMessage>,    // Group messages
    mls_group_data: Option<Vec<u8>>, // Serialized MLS group state
}
```

## MLS Protocol Integration

### Key Package Generation

The application generates key packages for each user during initialization:

```rust
fn init_user(&mut self, user: UserName) -> Result<()> {
    // Generate credential
    let credential = BasicCredential::new(user.to_string().into());
    
    // Generate signature key
    let signature_key = self.crypto.signature_key_generate(
        DefaultCiphersuite::signature_algorithm(),
        &credential.signature_key,
    )?;
    
    // Create key package
    let key_package = KeyPackage::builder()
        .build(
            CryptoConfig::default(),
            &self.crypto,
            &signature_key,
            Credential::Basic(credential),
        )?;
    
    // Store key package
    let key_package_bundle = KeyPackageBundle::from(key_package);
    self.key_packages.insert(user.to_string(), key_package_bundle);
    
    Ok(())
}
```

### Group Creation

Groups are created with proper MLS protocol setup:

```rust
fn create_group(&mut self, name: String) -> Result<()> {
    // Get user's key package
    let key_package_bundle = self.key_packages.get(&user.to_string())?;
    
    // Generate random group ID
    let group_id = GroupId::random(&self.crypto);
    
    // Create MLS group
    let group = MlsGroup::new(
        &self.crypto,
        &key_package_bundle.key_package(),
        &group_id,
        CryptoConfig::default(),
    )?;
    
    // Store group state
    let chat_group = ChatGroup {
        name: name.clone(),
        group_id: group_id.as_slice().to_vec().into_iter()
            .map(|b| format!("{:02x}", b)).collect::<String>(),
        members: vec![user.to_string()],
        messages: Vec::new(),
        mls_group_data: Some(group.save(&self.crypto)?),
    };
    
    self.groups.insert(name.clone(), chat_group);
    Ok(())
}
```

### Member Addition

Adding members involves secure key distribution:

```rust
fn add_member(&mut self, group_name: String, member: UserName) -> Result<()> {
    // Get member's key package
    let member_key_package = self.key_packages.get(&member.to_string())?;
    
    // Reconstruct MLS group from stored state
    let mut mls_group = MlsGroup::load(mls_group_data, &self.crypto)?;
    
    // Create add proposal
    let add_proposal = mls_group.create_add_proposal(
        &self.crypto,
        &member_key_package.key_package(),
    )?;
    
    // Create and apply commit
    let (commit, welcome, _group_info) = mls_group.create_commit(
        &self.crypto,
        &[add_proposal],
    )?;
    
    mls_group.apply_commit(&self.crypto, &commit)?;
    
    // Update group state
    group.members.push(member.to_string());
    group.mls_group_data = Some(mls_group.save(&self.crypto)?);
    
    Ok(())
}
```

### Message Encryption

Messages are encrypted using the MLS protocol:

```rust
fn send_message(&mut self, group_name: String, content: String) -> Result<()> {
    // Reconstruct MLS group
    let mut mls_group = MlsGroup::load(mls_group_data, &self.crypto)?;
    
    // Encrypt message using MLS
    let encrypted_message = mls_group.create_application_message(
        &self.crypto,
        &[],
        content.as_bytes(),
    )?;
    
    // Store message metadata
    let chat_message = ChatMessage {
        id: Uuid::new_v4().to_string(),
        sender: user.to_string(),
        content,
        timestamp: Utc::now(),
        group_id: group.group_id.clone(),
    };
    
    group.messages.push(chat_message);
    group.mls_group_data = Some(mls_group.save(&self.crypto)?);
    
    Ok(())
}
```

## Cryptographic Agility

### Modular Crypto Provider

The application uses `OpenMlsRustCrypto` as the cryptographic provider, which supports:

- **Key Encapsulation Mechanisms (KEMs)**: X25519, P-256, P-384, P-521
- **Authenticated Encryption**: AES-GCM, ChaCha20-Poly1305
- **Digital Signatures**: Ed25519, ECDSA with various curves
- **Hash Functions**: SHA-256, SHA-384, SHA-512

### Ciphersuite Configuration

The application uses the default ciphersuite configuration:

```rust
CryptoConfig::default()
```

This can be customized to use specific cryptographic algorithms:

```rust
let config = CryptoConfig {
    ciphersuite: Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519,
    version: ProtocolVersion::Mls10,
};
```

## State Management

### Persistence Strategy

The application uses JSON-based persistence for simplicity:

```rust
fn save_state(&self) -> Result<()> {
    let state_file = format!("{}/app_state.json", self.data_dir);
    let state = serde_json::to_string_pretty(&self.groups)?;
    fs::write(state_file, state)?;
    Ok(())
}

fn load_state(&mut self) -> Result<()> {
    let state_file = format!("{}/app_state.json", self.data_dir);
    if Path::new(&state_file).exists() {
        let data = fs::read_to_string(state_file)?;
        self.groups = serde_json::from_str(&data)?;
    }
    Ok(())
}
```

### Data Serialization

All data structures implement `Serialize` and `Deserialize` traits:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    // ... fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatGroup {
    // ... fields
}
```

## Error Handling

### Error Types

The application uses `anyhow::Result` for error handling:

```rust
use anyhow::{Context, Result};

fn some_function() -> Result<()> {
    let data = fs::read_to_string("file.txt")
        .context("Failed to read file")?;
    Ok(())
}
```

### Common Error Patterns

1. **Context Wrapping**: Add context to errors for better debugging
2. **Early Returns**: Use `?` operator for early error returns
3. **User-Friendly Messages**: Provide clear error messages to users

## CLI Interface

### Command Structure

The CLI uses `clap` for argument parsing:

```rust
#[derive(Parser)]
#[command(name = "mls-chat")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { user: UserName },
    CreateGroup { name: String },
    AddMember { group: String, member: UserName },
    Send { group: String, message: String },
    List { group: String },
    Info { group: String },
}
```

### User Input Validation

User names are restricted to Alice and Bob for demonstration:

```rust
#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum UserName {
    Alice,
    Bob,
}
```

## Development Guidelines

### Code Style

1. **Rust Conventions**: Follow Rust coding conventions
2. **Documentation**: Document all public functions and structs
3. **Error Handling**: Use proper error handling with context
4. **Testing**: Write unit tests for critical functions

### Security Considerations

1. **Key Management**: Never log or expose private keys
2. **Memory Safety**: Use secure memory handling for sensitive data
3. **Input Validation**: Validate all user inputs
4. **Error Messages**: Don't leak sensitive information in error messages

### Performance Considerations

1. **Efficient Serialization**: Use efficient serialization for large data
2. **Memory Usage**: Monitor memory usage for large groups
3. **Crypto Operations**: Minimize expensive cryptographic operations

## Testing

### Unit Tests

Add unit tests for critical functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_initialization() {
        let mut app = MlsChatApp::new().unwrap();
        app.init_user(UserName::Alice).unwrap();
        assert!(app.current_user.is_some());
    }

    #[test]
    fn test_group_creation() {
        let mut app = MlsChatApp::new().unwrap();
        app.init_user(UserName::Alice).unwrap();
        app.create_group("TestGroup".to_string()).unwrap();
        assert!(app.groups.contains_key("TestGroup"));
    }
}
```

### Integration Tests

Test the complete workflow:

```rust
#[tokio::test]
async fn test_complete_workflow() {
    let mut app = MlsChatApp::new().unwrap();
    
    // Initialize users
    app.init_user(UserName::Alice).unwrap();
    app.init_user(UserName::Bob).unwrap();
    
    // Create group
    app.create_group("TestGroup".to_string()).unwrap();
    
    // Add member
    app.add_member("TestGroup".to_string(), UserName::Bob).unwrap();
    
    // Send message
    app.send_message("TestGroup".to_string(), "Hello".to_string()).unwrap();
    
    // Verify message
    let group = app.groups.get("TestGroup").unwrap();
    assert_eq!(group.messages.len(), 1);
    assert_eq!(group.messages[0].content, "Hello");
}
```

## Debugging

### Debug Output

Enable debug logging:

```bash
RUST_LOG=debug cargo run -- <command>
```

### Common Debug Scenarios

1. **Key Package Issues**: Check key package generation and storage
2. **Group State Corruption**: Verify MLS group serialization/deserialization
3. **Message Encryption**: Debug message encryption process
4. **State Persistence**: Check file I/O operations

## Future Enhancements

### Planned Features

1. **Multi-User Support**: Extend beyond Alice and Bob
2. **Network Communication**: Add client-server architecture
3. **Message Decryption**: Implement full message decryption
4. **Key Rotation**: Automatic key rotation mechanisms
5. **Member Removal**: Remove members from groups
6. **Audit Logging**: Comprehensive security audit logs

### Architecture Improvements

1. **Modular Design**: Split into separate modules
2. **Plugin System**: Support for different crypto providers
3. **Configuration Management**: External configuration files
4. **Database Backend**: Replace JSON with proper database
5. **API Layer**: RESTful API for integration

## Contributing

### Development Setup

1. Clone the repository
2. Install Rust toolchain
3. Run `cargo build` to verify setup
4. Run tests with `cargo test`

### Code Review Process

1. Create feature branch
2. Implement changes with tests
3. Update documentation
4. Submit pull request
5. Address review comments

### Testing Requirements

1. Unit tests for new functions
2. Integration tests for workflows
3. Performance tests for critical paths
4. Security tests for crypto operations

---

This documentation provides a comprehensive guide for developers working on the MLS Chat application. For additional information, refer to the OpenMLS documentation and MLS protocol specification. 