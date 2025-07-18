# User Guide - MLS Chat Application

## Introduction

Welcome to MLS Chat, a secure messaging application that uses the Messaging Layer Security (MLS) protocol to provide end-to-end encryption for group communications. This guide will help you get started and use all the features of the application.

## What is MLS?

Messaging Layer Security (MLS) is a protocol that provides end-to-end encryption for group messaging. It ensures that:

- **Only group members can read messages** - Messages are encrypted and can only be decrypted by authorized group members
- **Forward secrecy** - Even if someone gets your current keys, they can't read your past messages
- **Post-compromise security** - If your device is compromised, you can recover security by updating keys
- **Efficient group key management** - Keys are automatically distributed and updated when group membership changes

## Getting Started

### Prerequisites

Before using MLS Chat, ensure you have:

- **Rust installed** (version 1.70 or higher)
- **Linux environment** (tested on Ubuntu 22.04)
- **Command line access** (terminal or SSH)

### Installation

1. **Clone the repository** (if you haven't already):
   ```bash
   git clone https://github.com/yourusername/mls-chat.git
   cd mls-chat
   ```

2. **Build the application**:
   ```bash
   cargo build --release
   ```

3. **Verify installation**:
   ```bash
   cargo run -- --help
   ```

You should see the help message with available commands.

## Basic Usage

### Step 1: Initialize Users

Before you can use the application, you need to initialize user identities. The application supports two users: Alice and Bob.

**Initialize Alice:**
```bash
cargo run -- init alice
```

**Initialize Bob:**
```bash
cargo run -- init bob
```

**What happens during initialization:**
- A cryptographic identity is created for the user
- A key package is generated containing public keys and credentials
- The user's cryptographic material is stored securely

**Expected output:**
```
Initializing user identity...
✅ User 'Alice' initialized successfully
```

### Step 2: Create a Group

Groups are the foundation of MLS Chat. Each group has its own encryption keys and membership.

**Create a group (as Alice):**
```bash
cargo run -- create-group "ProjectTeam"
```

**What happens when creating a group:**
- A new MLS group is created with a unique identifier
- The creator (Alice) becomes the first member
- Group encryption keys are generated
- The group state is saved locally

**Expected output:**
```
Creating new MLS group...
✅ Group 'ProjectTeam' created successfully
```

### Step 3: Add Members to the Group

Once you have a group, you can add other users as members.

**Add Bob to the group:**
```bash
cargo run -- add-member "ProjectTeam" bob
```

**What happens when adding a member:**
- Bob's key package is retrieved
- A cryptographic proposal is created to add Bob
- The proposal is committed, updating the group's encryption keys
- Bob receives the new group keys securely
- The updated group state is saved

**Expected output:**
```
Adding member to group...
✅ Member 'Bob' added to group 'ProjectTeam'
```

### Step 4: Send Messages

Now you can send encrypted messages to the group.

**Send a message:**
```bash
cargo run -- send "ProjectTeam" "Hello everyone! This is an encrypted message."
```

**What happens when sending a message:**
- The message is encrypted using the group's current encryption keys
- The encrypted message is created using the MLS protocol
- Message metadata (sender, timestamp, etc.) is stored
- The group state is updated to reflect the new message

**Expected output:**
```
Sending encrypted message...
✅ Message sent successfully
```

### Step 5: View Messages

You can view all messages in a group to see the conversation history.

**List all messages:**
```bash
cargo run -- list "ProjectTeam"
```

**Expected output:**
```
Messages in group 'ProjectTeam':
==================================================
[14:30:25] Alice: Hello everyone! This is an encrypted message.
```

### Step 6: View Group Information

Get detailed information about a group, including members and message count.

**Show group info:**
```bash
cargo run -- info "ProjectTeam"
```

**Expected output:**
```
Group: ProjectTeam
==============================
Group ID: a1b2c3d4e5f6...
Members: Alice, Bob
Message count: 1
```

## Advanced Usage

### Working with Multiple Groups

You can create and manage multiple groups simultaneously:

```bash
# Create a work group
cargo run -- create-group "WorkTeam"

# Create a personal group
cargo run -- create-group "Friends"

# Add Bob to both groups
cargo run -- add-member "WorkTeam" bob
cargo run -- add-member "Friends" bob

# Send messages to different groups
cargo run -- send "WorkTeam" "Meeting at 3 PM"
cargo run -- send "Friends" "Let's grab coffee!"
```

### Message Examples

Here are some examples of different types of messages you can send:

**Simple text:**
```bash
cargo run -- send "ProjectTeam" "Hello world!"
```

**Long messages:**
```bash
cargo run -- send "ProjectTeam" "This is a longer message that demonstrates how the application handles multi-line text and longer content. The MLS protocol ensures that all messages, regardless of length, are properly encrypted and secured."
```

**Messages with special characters:**
```bash
cargo run -- send "ProjectTeam" "Message with symbols: @#$%^&*() and emojis: 😀🎉🚀"
```

**Code snippets:**
```bash
cargo run -- send "ProjectTeam" "Here's the code: fn main() { println!(\"Hello, world!\"); }"
```

## Security Features

### What Makes MLS Chat Secure?

1. **End-to-End Encryption**: Messages are encrypted on your device and only decrypted on recipient devices
2. **Group Key Management**: Each group has its own encryption keys that are automatically managed
3. **Forward Secrecy**: Even if someone gets your current keys, they can't read past messages
4. **Post-Compromise Security**: If your device is compromised, you can recover by updating keys
5. **Message Authentication**: All messages are cryptographically signed to prevent tampering

### Understanding the Security Model

- **Local Storage**: All data is stored locally on your machine
- **No Network Communication**: This demo version doesn't send data over the network
- **Key Packages**: Each user has a key package containing their public keys and credentials
- **Group States**: Each group maintains its own cryptographic state

## Troubleshooting

### Common Issues and Solutions

#### Issue 1: "No user initialized" Error

**Problem:**
```
Error: No user initialized
```

**Cause:** You're trying to use a command without first initializing a user.

**Solution:**
```bash
# Initialize a user first
cargo run -- init alice
# Then try your command again
cargo run -- create-group "MyGroup"
```

#### Issue 2: "Group not found" Error

**Problem:**
```
Error: Group not found
```

**Cause:** You're trying to access a group that doesn't exist.

**Solution:**
```bash
# First, create the group
cargo run -- create-group "MyGroup"
# Then try your command again
cargo run -- send "MyGroup" "Hello"
```

#### Issue 3: "Key package not found" Error

**Problem:**
```
Error: Key package not found
```

**Cause:** You're trying to add a user who hasn't been initialized.

**Solution:**
```bash
# Initialize the user first
cargo run -- init bob
# Then add them to the group
cargo run -- add-member "MyGroup" bob
```

#### Issue 4: Build Errors

**Problem:**
```
error: failed to compile
```

**Cause:** Missing dependencies or incompatible Rust version.

**Solution:**
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

#### Issue 5: Permission Errors

**Problem:**
```
Error: Failed to create data directory
```

**Cause:** Insufficient permissions to create files.

**Solution:**
```bash
# Check current directory permissions
ls -la

# Ensure you have write permissions
chmod 755 .

# Or run with appropriate permissions
sudo cargo run -- init alice
```

### Debug Mode

If you're experiencing issues, you can enable debug mode for more detailed output:

```bash
RUST_LOG=debug cargo run -- <command>
```

This will show additional information about what the application is doing internally.

### Data Recovery

If your application state becomes corrupted:

1. **Stop the application** if it's running
2. **Backup your data**:
   ```bash
   cp -r mls_chat_data mls_chat_data_backup
   ```
3. **Delete the corrupted state**:
   ```bash
   rm mls_chat_data/app_state.json
   ```
4. **Restart the application** and reinitialize:
   ```bash
   cargo run -- init alice
   cargo run -- init bob
   cargo run -- create-group "MyGroup"
   ```

## Best Practices

### Security Best Practices

1. **Keep Dependencies Updated**: Regularly update Rust and dependencies for security patches
2. **Secure Your Machine**: Ensure your computer is secure since keys are stored locally
3. **Backup Important Data**: Regularly backup the `mls_chat_data` directory
4. **Use Strong Passwords**: If you implement additional authentication, use strong passwords

### Usage Best Practices

1. **Initialize Users First**: Always initialize users before creating groups
2. **Use Descriptive Group Names**: Choose meaningful names for your groups
3. **Regular Backups**: Backup your data regularly to prevent loss
4. **Test with Small Groups**: Start with small groups to understand the behavior

### Performance Tips

1. **Limit Group Size**: Large groups may impact performance
2. **Regular Cleanup**: Remove unused groups to free up resources
3. **Monitor Storage**: Keep an eye on the size of the `mls_chat_data` directory

## Limitations

### Current Limitations

1. **Two Users Only**: Currently supports only Alice and Bob for demonstration
2. **Local Storage**: All data is stored locally (no network communication)
3. **No Message Decryption**: Messages are encrypted but not decrypted in this demo
4. **Single Session**: No support for multiple concurrent sessions
5. **No Message History**: Messages are stored but not fully decrypted for display

### Understanding the Demo Nature

This application is designed for educational and demonstration purposes. In a production environment, you would need:

- **Network Communication**: Client-server architecture for real-time messaging
- **Message Decryption**: Full message decryption and display
- **Multi-User Support**: Support for arbitrary user names
- **Key Backup**: Secure key backup and recovery mechanisms
- **Audit Logging**: Comprehensive security audit logs

## Getting Help

### Where to Find Help

1. **This User Guide**: Check this guide for common issues and solutions
2. **README.md**: Main project documentation
3. **Developer Documentation**: Technical details in `docs/DEVELOPER.md`
4. **GitHub Issues**: Report bugs or request features on GitHub
5. **MLS Protocol Documentation**: Learn more about the underlying protocol

### Reporting Issues

When reporting issues, please include:

1. **Command used**: The exact command that caused the issue
2. **Error message**: The complete error message
3. **System information**: Your operating system and Rust version
4. **Steps to reproduce**: How to reproduce the issue
5. **Expected behavior**: What you expected to happen

### Example Issue Report

```
**Issue**: Cannot create group after user initialization

**Command**: cargo run -- create-group "TestGroup"

**Error**: Error: No user initialized

**System**: Ubuntu 22.04, Rust 1.70.0

**Steps to reproduce**:
1. Run: cargo run -- init alice
2. Run: cargo run -- create-group "TestGroup"

**Expected**: Group should be created successfully
```

## Conclusion

MLS Chat provides a secure foundation for end-to-end encrypted messaging using the MLS protocol. While this is a demonstration application, it showcases the key concepts and capabilities of MLS-based secure messaging.

For production use, consider implementing additional security measures, network communication, and proper key management systems.

Happy secure messaging! 🔐 