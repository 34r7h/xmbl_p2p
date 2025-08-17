#!/bin/bash

echo "🚀 Starting XMBL P2P Network..."
echo "================================="
echo ""

# Kill any existing processes
echo "🧹 Cleaning up existing processes..."
pkill -f xmbl_p2p_node 2>/dev/null || true
pkill -f xmbl_web_api 2>/dev/null || true
pkill -f xmbl_simulator 2>/dev/null || true
pkill -f xmbl_monitoring 2>/dev/null || true

echo "✅ Cleanup complete"
echo ""

# Start P2P nodes
echo "🌐 Starting P2P nodes..."

echo "1️⃣ Starting Node 1 (port 3001)..."
cargo run -p xmbl_p2p_node -- node_001 3001 100 &
NODE1_PID=$!
sleep 2

echo "2️⃣ Starting Node 2 (port 3002)..."
cargo run -p xmbl_p2p_node -- node_002 3002 150 &
NODE2_PID=$!
sleep 2

echo "3️⃣ Starting Node 3 (port 3003)..."
cargo run -p xmbl_p2p_node -- node_003 3003 200 &
NODE3_PID=$!
sleep 2

echo "4️⃣ Starting Node 4 (port 3004)..."
cargo run -p xmbl_p2p_node -- node_004 3004 250 &
NODE4_PID=$!
sleep 2

echo ""
echo "✅ All P2P nodes started!"
echo ""

# Wait a moment for nodes to discover each other
echo "⏳ Waiting for nodes to discover peers..."
sleep 5

echo ""
echo "🌐 P2P Network Status:"
echo "======================"

# Check if nodes are running
if ps -p $NODE1_PID > /dev/null; then
    echo "✅ Node 1 (node_001) - Port 3001 - PID: $NODE1_PID"
else
    echo "❌ Node 1 failed to start"
fi

if ps -p $NODE2_PID > /dev/null; then
    echo "✅ Node 2 (node_002) - Port 3002 - PID: $NODE2_PID"
else
    echo "❌ Node 2 failed to start"
fi

if ps -p $NODE3_PID > /dev/null; then
    echo "✅ Node 3 (node_003) - Port 3003 - PID: $NODE3_PID"
else
    echo "❌ Node 3 failed to start"
fi

if ps -p $NODE4_PID > /dev/null; then
    echo "✅ Node 4 (node_004) - Port 3004 - PID: $NODE4_PID"
else
    echo "❌ Node 4 failed to start"
fi

echo ""
echo "🔗 Network Ports:"
echo "================="
echo "Node 1: http://localhost:3001"
echo "Node 2: http://localhost:3002"
echo "Node 3: http://localhost:3003"
echo "Node 4: http://localhost:3004"
echo ""

echo "📱 P2P Client: http://localhost:3000/p2p-client.html"
echo ""

echo "🎯 Test the P2P Network:"
echo "========================"
echo "1. Open the P2P client in your browser"
echo "2. Upload files to see them distributed across nodes"
echo "3. Submit compute tasks to see them executed on peers"
echo "4. Watch real P2P communication between nodes"
echo ""

echo "🔄 Network will continue running..."
echo "Press Ctrl+C to stop all nodes"
echo ""

# Function to cleanup on exit
cleanup() {
    echo ""
    echo "🛑 Stopping P2P network..."
    kill $NODE1_PID 2>/dev/null || true
    kill $NODE2_PID 2>/dev/null || true
    kill $NODE3_PID 2>/dev/null || true
    kill $NODE4_PID 2>/dev/null || true
    echo "✅ All nodes stopped"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Keep script running
wait
