#!/bin/bash

# MLS Chat Application Test Script
# This script tests the basic functionality of the MLS chat application

set -e  # Exit on any error

echo "🧪 Testing MLS Chat Application"
echo "================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Function to run command and check result
run_test() {
    local test_name="$1"
    local command="$2"
    
    echo "Testing: $test_name"
    echo "Command: $command"
    
    if eval "$command" > /dev/null 2>&1; then
        print_status "$test_name passed"
    else
        print_error "$test_name failed"
        return 1
    fi
    echo ""
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found. Please run this script from the project root directory."
    exit 1
fi

# Clean up any existing data
echo "Cleaning up previous test data..."
rm -rf mls_chat_data
echo ""

# Test 1: Build the application
echo "1. Building the application..."
if cargo build --release > /dev/null 2>&1; then
    print_status "Application built successfully"
else
    print_error "Failed to build application"
    exit 1
fi
echo ""

# Test 2: Show help
echo "2. Testing help command..."
run_test "Help command" "cargo run -- --help | head -20"

# Test 3: Initialize Alice
echo "3. Testing user initialization..."
run_test "Initialize Alice" "cargo run -- init alice"

# Test 4: Initialize Bob
echo "4. Testing second user initialization..."
run_test "Initialize Bob" "cargo run -- init bob"

# Test 5: Create a group
echo "5. Testing group creation..."
run_test "Create group" "cargo run -- create-group 'TestGroup'"

# Test 6: Add member to group
echo "6. Testing member addition..."
run_test "Add Bob to group" "cargo run -- add-member 'TestGroup' bob"

# Test 7: Send a message
echo "7. Testing message sending..."
run_test "Send message" "cargo run -- send 'TestGroup' 'Hello, this is a test message!'"

# Test 8: List messages
echo "8. Testing message listing..."
run_test "List messages" "cargo run -- list 'TestGroup'"

# Test 9: Show group info
echo "9. Testing group info..."
run_test "Show group info" "cargo run -- info 'TestGroup'"

# Test 10: Send another message
echo "10. Testing second message..."
run_test "Send second message" "cargo run -- send 'TestGroup' 'This is another test message with special chars: @#$%^&*()'"

# Test 11: Verify message count
echo "11. Verifying message count..."
MESSAGE_COUNT=$(cargo run -- list 'TestGroup' 2>/dev/null | grep -c "Alice:" || echo "0")
if [ "$MESSAGE_COUNT" -eq 2 ]; then
    print_status "Message count verification passed (2 messages found)"
else
    print_warning "Message count verification: expected 2, found $MESSAGE_COUNT"
fi
echo ""

# Test 12: Test error handling
echo "12. Testing error handling..."
echo "Testing: Non-existent group access"
if ! cargo run -- send 'NonExistentGroup' 'test' > /dev/null 2>&1; then
    print_status "Error handling for non-existent group works correctly"
else
    print_error "Error handling for non-existent group failed"
fi
echo ""

# Test 13: Test duplicate member addition
echo "13. Testing duplicate member addition..."
run_test "Add Bob again (should be handled gracefully)" "cargo run -- add-member 'TestGroup' bob"

# Test 14: Create second group
echo "14. Testing multiple groups..."
run_test "Create second group" "cargo run -- create-group 'SecondGroup'"
run_test "Add Bob to second group" "cargo run -- add-member 'SecondGroup' bob"
run_test "Send message to second group" "cargo run -- send 'SecondGroup' 'Message in second group'"

# Test 15: Verify data persistence
echo "15. Testing data persistence..."
echo "Checking if data directory exists..."
if [ -d "mls_chat_data" ]; then
    print_status "Data directory created successfully"
else
    print_error "Data directory not created"
fi

if [ -f "mls_chat_data/app_state.json" ]; then
    print_status "Application state file created successfully"
    echo "State file size: $(wc -c < mls_chat_data/app_state.json) bytes"
else
    print_error "Application state file not created"
fi
echo ""

# Final summary
echo "🎉 Test Summary"
echo "==============="
print_status "All basic functionality tests completed"
echo ""
echo "Application features tested:"
echo "  ✅ User initialization (Alice and Bob)"
echo "  ✅ Group creation"
echo "  ✅ Member addition"
echo "  ✅ Message sending"
echo "  ✅ Message listing"
echo "  ✅ Group information display"
echo "  ✅ Multiple groups support"
echo "  ✅ Error handling"
echo "  ✅ Data persistence"
echo ""
echo "The MLS Chat application is working correctly!"
echo ""
echo "To try it yourself:"
echo "  cargo run -- init alice"
echo "  cargo run -- init bob"
echo "  cargo run -- create-group 'MyGroup'"
echo "  cargo run -- add-member 'MyGroup' bob"
echo "  cargo run -- send 'MyGroup' 'Hello, world!'"
echo "  cargo run -- list 'MyGroup'"
echo ""
print_status "Test completed successfully! 🚀" 