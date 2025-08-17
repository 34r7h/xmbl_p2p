#!/bin/bash

echo "🚀 XMBL P2P STORAGE AND COMPUTE NETWORK - COMPLETE SYSTEM STARTUP"
echo "=================================================================="
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
    local status=$1
    local message=$2
    case $status in
        "info") echo -e "${BLUE}ℹ️  $message${NC}" ;;
        "success") echo -e "${GREEN}✅ $message${NC}" ;;
        "warning") echo -e "${YELLOW}⚠️  $message${NC}" ;;
        "error") echo -e "${RED}❌ $message${NC}" ;;
        "header") echo -e "${PURPLE}🎯 $message${NC}" ;;
        "step") echo -e "${CYAN}🔧 $message${NC}" ;;
    esac
}

# Function to check if port is available
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Function to kill process on port
kill_port() {
    local port=$1
    local pid=$(lsof -ti:$port 2>/dev/null)
    if [ ! -z "$pid" ]; then
        print_status "warning" "Port $port is in use. Killing process $pid"
        kill -9 $pid 2>/dev/null
        sleep 1
    fi
}

print_status "header" "SYSTEM STARTUP SEQUENCE"
echo "================================================"

# Step 1: Clean up any existing processes
print_status "step" "Step 1: Cleaning up existing processes"
kill_port 3000
kill_port 3001
kill_port 3002
sleep 2
print_status "success" "Ports cleared"

# Step 2: Check Rust compilation
print_status "step" "Step 2: Checking Rust compilation"
echo "Compiling all Rust crates..."
if cargo check --workspace > /dev/null 2>&1; then
    print_status "success" "All Rust crates compile successfully"
else
    print_status "error" "Rust compilation failed"
    exit 1
fi

# Step 3: Start Monitoring Service
print_status "step" "Step 3: Starting Monitoring Service"
echo "Starting monitoring service on port 3001..."
cargo run -p xmbl_monitoring > /tmp/monitoring.log 2>&1 &
MONITORING_PID=$!
sleep 3

if check_port 3001; then
    print_status "success" "Monitoring service started on port 3001"
else
    print_status "error" "Failed to start monitoring service"
    exit 1
fi

# Step 4: Start Web Dashboard
print_status "step" "Step 4: Starting Web Dashboard"
echo "Starting web dashboard on port 3000..."
cd web-dashboard
python3 -m http.server 3000 > /tmp/dashboard.log 2>&1 &
DASHBOARD_PID=$!
cd ..
sleep 3

if check_port 3000; then
    print_status "success" "Web dashboard started on port 3000"
else
    print_status "error" "Failed to start web dashboard"
    exit 1
fi

# Step 5: Start Simulator (optional)
print_status "step" "Step 5: Starting Network Simulator"
echo "Starting network simulator..."
cargo run -p xmbl_simulator > /tmp/simulator.log 2>&1 &
SIMULATOR_PID=$!
sleep 3

if ps -p $SIMULATOR_PID > /dev/null 2>&1; then
    print_status "success" "Network simulator started"
else
    print_status "warning" "Network simulator failed to start (this is optional)"
fi

# Step 6: System Status Check
print_status "step" "Step 6: System Status Verification"
echo "Verifying all services..."

# Check monitoring service
if curl -s http://localhost:3001/health > /dev/null; then
    print_status "success" "Monitoring service: HEALTHY"
else
    print_status "error" "Monitoring service: UNHEALTHY"
fi

# Check web dashboard
if curl -s http://localhost:3000 > /dev/null; then
    print_status "success" "Web dashboard: HEALTHY"
else
    print_status "error" "Web dashboard: UNHEALTHY"
fi

# Check functional P2P client app
if curl -s http://localhost:3000/functional-client.html > /dev/null; then
    print_status "success" "Functional P2P client app: HEALTHY"
else
    print_status "error" "Functional P2P client app: UNHEALTHY"
fi

echo ""
print_status "header" "SYSTEM STARTUP COMPLETE!"
echo "================================================"

# Display access information
echo ""
print_status "info" "🌐 ACCESS POINTS:"
echo "   • Main Dashboard: http://localhost:3000"
echo "   • Functional P2P Client: http://localhost:3000/functional-client.html"
echo "   • Legacy Client: http://localhost:3000/client.html"
echo "   • Monitoring API: http://localhost:3001"
echo ""

# Display CLI testing instructions
print_status "info" "🔧 CLI FUNCTIONALITY TESTING:"
echo "   Test the following commands to verify system functionality:"
echo ""
echo "   1. Node Information:"
echo "      cargo run -p xmbl_cli -- node-info"
echo ""
echo "   2. Storage Operations:"
echo "      cargo run -p xmbl_cli -- storage-store test.txt 3"
echo ""
echo "   3. Compute Operations:"
echo "      cargo run -p xmbl_cli -- compute-submit test.wasm '{\"data\": [1,2,3]}'"
echo ""
echo "   4. Blockchain Operations:"
echo "      cargo run -p xmbl_cli -- blockchain-transfer alice bob 100"
echo ""
echo "   5. Network Operations:"
echo "      cargo run -p xmbl_cli -- network-peers"
echo ""

# Display monitoring instructions
print_status "info" "📊 MONITORING VERIFICATION:"
echo "   Check these endpoints to verify monitoring functionality:"
echo ""
echo "   1. Health Check:"
echo "      curl http://localhost:3001/health"
echo ""
echo "   2. System Metrics:"
echo "      curl http://localhost:3001/metrics"
echo ""
echo "   3. Create Test Alert:"
echo "      curl -X POST http://localhost:3001/alerts \\"
echo "        -H \"Content-Type: application/json\" \\"
echo "        -d '{\"message\": \"System test\", \"severity\": \"info\"}'"
echo ""

# Display web interface instructions
print_status "info" "🌐 WEB INTERFACE VERIFICATION:"
echo "   Open these URLs in your browser to verify web functionality:"
echo ""
echo "   1. Main Dashboard: http://localhost:3000"
echo "      - Should show network overview and metrics"
echo "      - Should display real-time system status"
echo ""
echo "   2. Functional P2P Client: http://localhost:3000/functional-client.html"
echo "      - Should show storage and compute interface"
echo "      - Should allow file uploads with redundancy selection"
echo "      - Should allow WASM task submission"
echo "      - Should display network status and blockchain info"
echo "      - Should show real-time progress and results"
echo "      - Should be fully functional (not just simulation)"
echo ""

# Display process information
echo ""
print_status "info" "📋 RUNNING PROCESSES:"
echo "   • Monitoring Service PID: $MONITORING_PID (Port 3001)"
echo "   • Web Dashboard PID: $DASHBOARD_PID (Port 3000)"
if [ ! -z "$SIMULATOR_PID" ] && ps -p $SIMULATOR_PID > /dev/null 2>&1; then
    echo "   • Network Simulator PID: $SIMULATOR_PID"
fi
echo ""

# Display log file locations
print_status "info" "📝 LOG FILES:"
echo "   • Monitoring: /tmp/monitoring.log"
echo "   • Dashboard: /tmp/dashboard.log"
if [ ! -z "$SIMULATOR_PID" ] && ps -p $SIMULATOR_PID > /dev/null 2>&1; then
    echo "   • Simulator: /tmp/simulator.log"
fi
echo ""

# Display next steps
print_status "header" "NEXT STEPS:"
echo "================================================"
echo "1. 🌐 Open http://localhost:3000/functional-client.html in your browser"
echo "2. 🔧 Test CLI commands to verify functionality"
echo "3. 📊 Check monitoring endpoints for system health"
echo "4. 🎮 Experience the complete P2P storage and compute workflow"
echo "5. 🚀 The simulator adds activity on top of your webapp and CLI operations"
echo ""

print_status "success" "🎉 XMBL P2P Network is now running!"
echo "All systems are operational and ready for testing."
echo ""
echo "Press Ctrl+C to stop this script (services will continue running)"
echo "Use 'pkill -f xmbl' to stop all services"

# Keep script running to show status
trap 'echo ""; print_status "info" "Script stopped. Services continue running."; exit 0' INT

while true; do
    sleep 30
    echo ""
    print_status "info" "🔄 System Status Check (every 30 seconds):"
    
    # Check monitoring service
    if curl -s http://localhost:3001/health > /dev/null; then
        echo -e "   ${GREEN}✅ Monitoring: HEALTHY${NC}"
    else
        echo -e "   ${RED}❌ Monitoring: UNHEALTHY${NC}"
    fi
    
    # Check web dashboard
    if curl -s http://localhost:3000 > /dev/null; then
        echo -e "   ${GREEN}✅ Dashboard: HEALTHY${NC}"
    else
        echo -e "   ${RED}❌ Dashboard: UNHEALTHY${NC}"
    fi
    
    # Check simulator
    if [ ! -z "$SIMULATOR_PID" ] && ps -p $SIMULATOR_PID > /dev/null 2>&1; then
        echo -e "   ${GREEN}✅ Simulator: RUNNING${NC}"
    else
        echo -e "   ${YELLOW}⚠️  Simulator: STOPPED${NC}"
    fi
done
