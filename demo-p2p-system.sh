#!/bin/bash

echo "🚀 XMBL P2P Network System Demo"
echo "================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_header() {
    echo -e "${PURPLE}🔹 $1${NC}"
}

print_success() {
    echo -e "${GREEN}🎉 $1${NC}"
}

# Check if required tools are installed
check_dependencies() {
    print_header "Checking dependencies..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found. Please install Rust first."
        exit 1
    fi
    
    if ! command -v python3 &> /dev/null; then
        print_error "Python3 not found. Please install Python3 first."
        exit 1
    fi
    
    if ! command -v curl &> /dev/null; then
        print_error "curl not found. Please install curl first."
        exit 1
    fi
    
    print_status "All dependencies are available"
}

# Clean up any existing processes
cleanup() {
    print_header "Cleaning up existing processes..."
    
    pkill -f xmbl_p2p_node 2>/dev/null || true
    pkill -f xmbl_p2p_proxy 2>/dev/null || true
    pkill -f "python3 -m http.server" 2>/dev/null || true
    
    print_status "Cleanup complete"
}

# Build the system
build_system() {
    print_header "Building P2P system components..."
    
    print_info "Building P2P nodes..."
    if cargo build -p xmbl_p2p_node; then
        print_status "P2P nodes built successfully"
    else
        print_error "Failed to build P2P nodes"
        exit 1
    fi
    
    print_info "Building P2P proxy..."
    if cargo build -p xmbl_p2p_proxy; then
        print_status "P2P proxy built successfully"
    else
        print_error "Failed to build P2P proxy"
        exit 1
    fi
    
    print_success "All components built successfully"
}

# Start the P2P network
start_p2p_network() {
    print_header "Starting P2P network..."
    
    print_info "Starting P2P nodes..."
    ./start-p2p-network.sh &
    P2P_NETWORK_PID=$!
    
    # Wait for nodes to start
    sleep 5
    
    if ps -p $P2P_NETWORK_PID > /dev/null; then
        print_status "P2P network started successfully"
    else
        print_error "Failed to start P2P network"
        exit 1
    fi
}

# Start the P2P proxy
start_p2p_proxy() {
    print_header "Starting P2P WebSocket proxy..."
    
    print_info "Starting proxy on port 3006..."
    cargo run -p xmbl_p2p_proxy &
    PROXY_PID=$!
    
    # Wait for proxy to start
    sleep 3
    
    if ps -p $PROXY_PID > /dev/null; then
        print_status "P2P proxy started successfully"
    else
        print_error "Failed to start P2P proxy"
        exit 1
    fi
}

# Start the web dashboard
start_web_dashboard() {
    print_header "Starting web dashboard..."
    
    print_info "Starting dashboard on port 3005..."
    cd web-dashboard && python3 -m http.server 3005 &
    DASHBOARD_PID=$!
    cd ..
    
    # Wait for dashboard to start
    sleep 2
    
    if ps -p $DASHBOARD_PID > /dev/null; then
        print_status "Web dashboard started successfully"
    else
        print_error "Failed to start web dashboard"
        exit 1
    fi
}

# Test the P2P system
test_p2p_system() {
    print_header "Testing P2P system functionality..."
    
    print_info "Testing P2P node connectivity..."
    
    # Test each node
    for port in 3001 3002 3003 3004; do
        if curl -s "http://localhost:$port" > /dev/null 2>&1; then
            print_status "Node on port $port is responding"
        else
            print_warning "Node on port $port is not responding (this is expected for TCP-only nodes)"
        fi
    done
    
    print_info "Testing web dashboard..."
    if curl -s "http://localhost:3005" > /dev/null 2>&1; then
        print_status "Web dashboard is accessible"
    else
        print_error "Web dashboard is not accessible"
        exit 1
    fi
    
    print_info "Testing P2P client..."
    if curl -s "http://localhost:3005/p2p-client.html" > /dev/null 2>&1; then
        print_status "P2P client is accessible"
    else
        print_error "P2P client is not accessible"
        exit 1
    fi
    
    print_info "Testing enhanced dashboard..."
    if curl -s "http://localhost:3005/enhanced-dashboard.html" > /dev/null 2>&1; then
        print_status "Enhanced dashboard is accessible"
    else
        print_error "Enhanced dashboard is not accessible"
        exit 1
    fi
}

# Show system status
show_system_status() {
    print_header "System Status Overview"
    echo ""
    
    echo "🌐 P2P Network Components:"
    echo "  • P2P Nodes: $(ps aux | grep xmbl_p2p_node | grep -v grep | wc -l | tr -d ' ') running"
    echo "  • P2P Proxy: $(ps aux | grep xmbl_p2p_proxy | grep -v grep | wc -l | tr -d ' ') running"
    echo "  • Web Dashboard: $(ps aux | grep "python3 -m http.server" | grep -v grep | wc -l | tr -d ' ') running"
    echo ""
    
    echo "🔗 Network Ports:"
    echo "  • P2P Nodes: 3001, 3002, 3003, 3004 (TCP)"
    echo "  • P2P Proxy: 3006 (WebSocket)"
    echo "  • Web Dashboard: 3005 (HTTP)"
    echo ""
    
    echo "📱 Access Points:"
    echo "  • P2P Client: http://localhost:3005/p2p-client.html"
    echo "  • Enhanced Dashboard: http://localhost:3005/enhanced-dashboard.html"
    echo ""
}

# Show demo instructions
show_demo_instructions() {
    print_header "Demo Instructions"
    echo ""
    
    echo "🎯 To test the P2P system:"
    echo ""
    echo "1. 📁 Test File Storage:"
    echo "   • Open http://localhost:3005/p2p-client.html"
    echo "   • Upload a file with desired redundancy"
    echo "   • Watch it get distributed across P2P nodes"
    echo ""
    
    echo "2. ⚡ Test Compute Execution:"
    echo "   • In the P2P client, submit a WASM file and input data"
    echo "   • Watch the compute task get executed on a P2P node"
    echo ""
    
    echo "3. 📊 Monitor Network:"
    echo "   • Open http://localhost:3005/enhanced-dashboard.html"
    echo "   • Watch real-time network status, storage distribution, and compute tasks"
    echo ""
    
    echo "4. 🔍 Check Node Logs:"
    echo "   • Look at the terminal where P2P nodes are running"
    echo "   • See real P2P communication and file storage"
    echo ""
    
    echo "🔄 The system will continue running until you stop it with Ctrl+C"
    echo ""
}

# Main demo function
main() {
    echo ""
    print_header "Starting XMBL P2P Network System Demo"
    echo ""
    
    # Check dependencies
    check_dependencies
    
    # Clean up
    cleanup
    
    # Build system
    build_system
    
    # Start components
    start_p2p_network
    start_p2p_proxy
    start_web_dashboard
    
    # Wait for everything to start
    sleep 3
    
    # Test system
    test_p2p_system
    
    # Show status
    show_system_status
    
    # Show demo instructions
    show_demo_instructions
    
    print_success "P2P Network System is now running and ready for demo!"
    echo ""
    echo "Press Ctrl+C to stop all components"
    echo ""
    
    # Keep script running and handle cleanup on exit
    trap 'echo ""; print_header "Stopping P2P Network System..."; cleanup; print_success "Demo completed!"; exit 0' INT TERM
    
    # Wait for user to stop
    wait
}

# Run the demo
main
