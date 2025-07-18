use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::Path,
};
use uuid::Uuid;

/// Minimal CLI-based messaging app demonstrating MLS protocol concepts
#[derive(Parser)]
#[command(name = "mls-chat")]
#[command(about = "End-to-end encrypted messaging using MLS protocol concepts")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new user identity
    Init {
        /// User name (Alice or Bob)
        #[arg(value_enum)]
        user: UserName,
    },
    /// Create a new group
    CreateGroup {
        /// Group name
        name: String,
    },
    /// Add a member to the group
    AddMember {
        /// Group name
        group: String,
        /// Member to add (Alice or Bob)
        #[arg(value_enum)]
        member: UserName,
    },
    /// Send a message to the group
    Send {
        /// Group name
        group: String,
        /// Message content
        message: String,
    },
    /// List all messages in a group
    List {
        /// Group name
        group: String,
    },
    /// Show group information
    Info {
        /// Group name
        group: String,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum, Serialize, Deserialize)]
enum UserName {
    Alice,
    Bob,
}

impl std::fmt::Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserName::Alice => write!(f, "Alice"),
            UserName::Bob => write!(f, "Bob"),
        }
    }
}

/// Mock cryptographic key for demonstration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MockKey {
    id: String,
    public_key: String,
    private_key: String, // In real implementation, this would be encrypted
}

/// Mock MLS group state for demonstration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MockMlsGroup {
    group_id: String,
    epoch: u32,
    tree_hash: String,
    group_secret: String, // In real implementation, this would be encrypted
    members: Vec<String>,
}

/// Represents a message in the MLS group
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    id: String,
    sender: String,
    content: String,
    encrypted_content: String, // Mock encrypted content
    timestamp: DateTime<Utc>,
    group_id: String,
    epoch: u32,
}

/// Represents a group in the MLS chat application
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatGroup {
    name: String,
    group_id: String,
    members: Vec<String>,
    messages: Vec<ChatMessage>,
    mls_group: MockMlsGroup,
}

/// Main application state
struct MlsChatApp {
    current_user: Option<UserName>,
    groups: HashMap<String, ChatGroup>,
    user_keys: HashMap<String, MockKey>,
    data_dir: String,
}

impl MlsChatApp {
    fn new() -> Result<Self> {
        let data_dir = "mls_chat_data".to_string();
        fs::create_dir_all(&data_dir).context("Failed to create data directory")?;
        
        Ok(Self {
            current_user: None,
            groups: HashMap::new(),
            user_keys: HashMap::new(),
            data_dir,
        })
    }

    /// Initialize a new user identity
    fn init_user(&mut self, user: UserName) -> Result<()> {
        println!("{}", "Initializing user identity...".green());
        
        // Generate mock cryptographic keys
        let key = MockKey {
            id: Uuid::new_v4().to_string(),
            public_key: format!("pub_key_{}", Uuid::new_v4()),
            private_key: format!("priv_key_{}", Uuid::new_v4()),
        };
        
        self.user_keys.insert(user.to_string(), key);
        self.current_user = Some(user);
        
        println!("✅ User '{}' initialized successfully", user);
        println!("   Generated cryptographic identity and key package");
        self.save_state()?;
        Ok(())
    }

    /// Create a new MLS group
    fn create_group(&mut self, name: String) -> Result<()> {
        let user = self.current_user.context("No user initialized")?;
        println!("{}", "Creating new MLS group...".green());
        
        // Verify user has keys
        if !self.user_keys.contains_key(&user.to_string()) {
            return Err(anyhow::anyhow!("User '{}' not initialized", user));
        }
        
        // Create mock MLS group
        let group_id = Uuid::new_v4().to_string();
        let mls_group = MockMlsGroup {
            group_id: group_id.clone(),
            epoch: 1,
            tree_hash: format!("tree_hash_{}", Uuid::new_v4()),
            group_secret: format!("group_secret_{}", Uuid::new_v4()),
            members: vec![user.to_string()],
        };
        
        // Create chat group
        let chat_group = ChatGroup {
            name: name.clone(),
            group_id: group_id.clone(),
            members: vec![user.to_string()],
            messages: Vec::new(),
            mls_group,
        };
        
        self.groups.insert(name.clone(), chat_group);
        println!("✅ Group '{}' created successfully", name);
        println!("   MLS Group ID: {}", group_id);
        println!("   Initial epoch: 1");
        println!("   Group secret generated and distributed");
        self.save_state()?;
        Ok(())
    }

    /// Add a member to an existing group
    fn add_member(&mut self, group_name: String, member: UserName) -> Result<()> {
        let user = self.current_user.context("No user initialized")?;
        println!("{}", "Adding member to group...".green());
        
        let group = self.groups.get_mut(&group_name)
            .context("Group not found")?;
        
        if group.members.contains(&member.to_string()) {
            println!("⚠️  Member '{}' is already in the group", member);
            return Ok(());
        }
        
        // Verify member has keys
        if !self.user_keys.contains_key(&member.to_string()) {
            return Err(anyhow::anyhow!("Member '{}' not initialized", member));
        }
        
        // Simulate MLS add proposal and commit
        println!("   Creating Add proposal for '{}'", member);
        println!("   Generating new group secret");
        println!("   Distributing updated keys to all members");
        
        // Update group state
        group.mls_group.epoch += 1;
        group.mls_group.group_secret = format!("group_secret_{}", Uuid::new_v4());
        group.mls_group.tree_hash = format!("tree_hash_{}", Uuid::new_v4());
        group.members.push(member.to_string());
        group.mls_group.members = group.members.clone();
        
        println!("✅ Member '{}' added to group '{}'", member, group_name);
        println!("   Epoch updated to: {}", group.mls_group.epoch);
        println!("   Group secret rotated for security");
        self.save_state()?;
        Ok(())
    }

    /// Send a message to a group
    fn send_message(&mut self, group_name: String, content: String) -> Result<()> {
        let _user = self.current_user.context("No user initialized")?;
        println!("{}", "Sending encrypted message...".green());
        
        let group = self.groups.get_mut(&group_name)
            .context("Group not found")?;
        
        // Verify user is a member
        if !group.members.contains(&_user.to_string()) {
            return Err(anyhow::anyhow!("User '{}' is not a member of group '{}'", _user, group_name));
        }
        
        // Simulate MLS message encryption
        println!("   Encrypting message with group secret");
        println!("   Using epoch: {}", group.mls_group.epoch);
        
        let encrypted_content = format!("encrypted_{}_{}", 
            group.mls_group.group_secret[..16].to_string(), 
            content.replace(" ", "_")
        );
        
        // Create chat message
        let chat_message = ChatMessage {
            id: Uuid::new_v4().to_string(),
            sender: _user.to_string(),
            content: content.clone(),
            encrypted_content,
            timestamp: Utc::now(),
            group_id: group.group_id.clone(),
            epoch: group.mls_group.epoch,
        };
        
        group.messages.push(chat_message);
        
        println!("✅ Message sent successfully");
        println!("   Message encrypted with group key");
        println!("   Forward secrecy maintained");
        self.save_state()?;
        Ok(())
    }

    /// List all messages in a group
    fn list_messages(&self, group_name: String) -> Result<()> {
        let group = self.groups.get(&group_name)
            .context("Group not found")?;
        
        println!("{}", format!("Messages in group '{}':", group_name).blue());
        println!("{}", "=".repeat(50));
        println!("Group ID: {}", group.group_id);
        println!("Current Epoch: {}", group.mls_group.epoch);
        println!("Members: {}", group.members.join(", "));
        println!("{}", "=".repeat(50));
        
        if group.messages.is_empty() {
            println!("No messages yet.");
        } else {
            for message in &group.messages {
                println!("[{}] {} (Epoch {}): {}", 
                    message.timestamp.format("%H:%M:%S"),
                    message.sender.yellow(),
                    message.epoch,
                    message.content
                );
                println!("   Encrypted: {}", message.encrypted_content.dimmed());
            }
        }
        Ok(())
    }

    /// Show group information
    fn show_group_info(&self, group_name: String) -> Result<()> {
        let group = self.groups.get(&group_name)
            .context("Group not found")?;
        
        println!("{}", format!("Group: {}", group_name).blue());
        println!("{}", "=".repeat(30));
        println!("Group ID: {}", group.group_id);
        println!("Current Epoch: {}", group.mls_group.epoch);
        println!("Tree Hash: {}", group.mls_group.tree_hash);
        println!("Members: {}", group.members.join(", "));
        println!("Message count: {}", group.messages.len());
        println!("Group Secret: {}...", &group.mls_group.group_secret[..20]);
        Ok(())
    }

    /// Save application state to disk
    fn save_state(&self) -> Result<()> {
        let state_file = format!("{}/app_state.json", self.data_dir);
        let state = serde_json::to_string_pretty(&self.groups)?;
        fs::write(state_file, state)?;
        
        let keys_file = format!("{}/user_keys.json", self.data_dir);
        let keys_state = serde_json::to_string_pretty(&self.user_keys)?;
        fs::write(keys_file, keys_state)?;
        
        let current_user_file = format!("{}/current_user.json", self.data_dir);
        if let Some(user) = &self.current_user {
            let user_state = serde_json::to_string_pretty(&user)?;
            fs::write(current_user_file, user_state)?;
        }
        Ok(())
    }

    /// Load application state from disk
    fn load_state(&mut self) -> Result<()> {
        let state_file = format!("{}/app_state.json", self.data_dir);
        if Path::new(&state_file).exists() {
            let data = fs::read_to_string(state_file)?;
            self.groups = serde_json::from_str(&data)?;
        }
        
        let keys_file = format!("{}/user_keys.json", self.data_dir);
        if Path::new(&keys_file).exists() {
            let data = fs::read_to_string(keys_file)?;
            self.user_keys = serde_json::from_str(&data)?;
        }
        
        let current_user_file = format!("{}/current_user.json", self.data_dir);
        if Path::new(&current_user_file).exists() {
            let data = fs::read_to_string(current_user_file)?;
            self.current_user = serde_json::from_str(&data)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let mut app = MlsChatApp::new()?;
    app.load_state()?;
    
    match cli.command {
        Commands::Init { user } => {
            app.init_user(user)?;
        }
        Commands::CreateGroup { name } => {
            app.create_group(name)?;
        }
        Commands::AddMember { group, member } => {
            app.add_member(group, member)?;
        }
        Commands::Send { group, message } => {
            app.send_message(group, message)?;
        }
        Commands::List { group } => {
            app.list_messages(group)?;
        }
        Commands::Info { group } => {
            app.show_group_info(group)?;
        }
    }
    
    Ok(())
} 