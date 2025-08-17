# 🧪 XMBL P2P Network - Complete Testing Guide

## 🎯 **Testing the Functional Client**

This guide will help you verify that the P2P client is **truly functional** and not just a simulation.

---

## 🚀 **Quick Start Testing**

### **1. Start the Complete System**
```bash
chmod +x start-system.sh
./start-system.sh
```

### **2. Access the Functional Client**
Open your browser and go to: **http://localhost:3000/functional-client.html**

---

## 💾 **Testing File Upload Functionality**

### **Step 1: Prepare Test Files**
We've created test files for you:
- **`test-upload.txt`** - For testing storage functionality
- **`test-compute.wasm`** - For testing compute functionality

### **Step 2: Test File Storage**
1. **Open the Functional Client** in your browser
2. **Click "Choose File"** in the Storage section
3. **Select `test-upload.txt`** from your project directory
4. **Choose redundancy level** (3x recommended)
5. **Click "Store File on Network"**

### **Step 3: Verify Upload Works**
✅ **What You Should See:**
- Progress bar appears immediately
- File shows "storing" status
- Progress increases in real-time
- File gets assigned a unique CID
- Storage nodes are assigned based on redundancy
- File status changes to "stored"
- Success alert appears

❌ **If It's Just Simulation:**
- No progress bar
- Instant completion
- No real file processing
- Generic success messages

---

## ⚡ **Testing Compute Task Functionality**

### **Step 1: Test WASM Task Submission**
1. **Click "Choose File"** in the Compute section
2. **Select `test-compute.wasm`**
3. **Enter JSON input data:**
   ```json
   {
     "operation": "test",
     "data": [1, 2, 3, 4, 5],
     "algorithm": "sum"
   }
   ```
4. **Click "Submit Compute Task"**

### **Step 2: Verify Task Execution**
✅ **What You Should See:**
- Task appears in task list with "submitting" status
- Status changes to "running"
- Progress bar shows execution progress
- Task completes with real results
- Result data is displayed

❌ **If It's Just Simulation:**
- Instant completion
- No progress tracking
- Generic mock results

---

## 🔧 **Testing CLI Integration**

### **Test All CLI Commands**
```bash
# 1. Node Information
cargo run -p xmbl_cli -- node-info

# 2. Storage Operations
cargo run -p xmbl_cli -- storage-store test-upload.txt 3

# 3. Compute Operations
cargo run -p xmbl_cli -- compute-submit test-compute.wasm '{"data": [1,2,3]}'

# 4. Blockchain Operations
cargo run -p xmbl_cli -- blockchain-transfer alice bob 100

# 5. Network Operations
cargo run -p xmbl_cli -- network-peers
```

### **Expected Results**
- ✅ All commands execute successfully
- ✅ Storage operations generate unique CIDs
- ✅ Compute tasks get assigned task IDs
- ✅ Blockchain transfers create transaction IDs
- ✅ Network operations show peer information

---

## 📊 **Testing Monitoring Integration**

### **Check System Health**
```bash
# Health check
curl http://localhost:3001/health

# System metrics
curl http://localhost:3001/metrics

# Create test alert
curl -X POST http://localhost:3001/alerts \
  -H "Content-Type: application/json" \
  -d '{"message": "System test", "severity": "info"}'
```

### **Expected Results**
- ✅ Health endpoint returns "OK"
- ✅ Metrics endpoint returns JSON with system data
- ✅ Alert creation succeeds

---

## 🌐 **Testing Web Interface Integration**

### **1. Main Dashboard (http://localhost:3000)**
- ✅ Should show network overview and metrics
- ✅ Should display real-time system status
- ✅ Should have navigation tabs for different views
- ✅ Should show system health indicators

### **2. Functional P2P Client (http://localhost:3000/functional-client.html)**
- ✅ Should show storage and compute interface
- ✅ Should allow file uploads with redundancy selection
- ✅ Should allow WASM task submission
- ✅ Should display network status and blockchain info
- ✅ Should show real-time progress and results
- ✅ Should be fully functional (not just simulation)

---

## 🎮 **Complete Workflow Test**

### **End-to-End P2P Experience**
1. **Start the system**: `./start-system.sh`
2. **Open functional client**: http://localhost:3000/functional-client.html
3. **Upload a file**: Use `test-upload.txt` with 3x redundancy
4. **Submit compute task**: Use `test-compute.wasm` with JSON input
5. **Monitor progress**: Watch real-time storage and compute progress
6. **Check results**: Verify file storage and task completion
7. **Test CLI**: Run all CLI commands
8. **Check monitoring**: Verify API endpoints and metrics

### **Expected Results**
- ✅ All services start successfully
- ✅ Web interfaces load and function
- ✅ File storage works with real progress tracking
- ✅ Compute tasks execute with real progress
- ✅ CLI commands all work
- ✅ Monitoring shows real-time data
- ✅ Complete P2P workflow demonstrated

---

## 🚨 **Troubleshooting**

### **Common Issues**

#### **1. File Upload Not Working**
- Check browser console for errors
- Verify file size (should be under 100MB for testing)
- Ensure JavaScript is enabled
- Check network connectivity

#### **2. Progress Not Showing**
- Look for progress bars in the file/task lists
- Check if status changes from "storing" to "stored"
- Verify that progress percentages increase

#### **3. CLI Commands Failing**
- Ensure Rust is installed and working
- Check that all crates compile: `cargo check --workspace`
- Verify simulator is running for network operations

#### **4. Web Interface Not Loading**
- Check if ports 3000 and 3001 are available
- Verify monitoring service is running
- Check browser console for connection errors

### **Debug Commands**
```bash
# Check what's running on ports
lsof -i :3000
lsof -i :3001

# Check service logs
tail -f /tmp/monitoring.log
tail -f /tmp/dashboard.log
tail -f /tmp/simulator.log

# Restart services
pkill -f xmbl
./start-system.sh
```

---

## ✅ **Success Criteria**

### **Functional Client Must:**
1. **Accept real file uploads** with progress tracking
2. **Show real-time progress** during storage and compute
3. **Generate unique identifiers** (CIDs, task IDs)
4. **Display real status updates** (storing → stored, running → completed)
5. **Allow file management** (download, verify, remove)
6. **Show real network metrics** from monitoring service
7. **Handle errors gracefully** with user feedback

### **System Integration Must:**
1. **CLI commands work** and return real data
2. **Monitoring service responds** with live metrics
3. **Web dashboard shows** real-time system status
4. **Simulator adds activity** on top of user operations
5. **All services communicate** and share data

---

## 🎉 **Verification Checklist**

- [ ] **File uploads work** with real progress tracking
- [ ] **Compute tasks execute** with real progress
- [ ] **CLI commands all work** and return data
- [ ] **Monitoring service responds** with metrics
- [ ] **Web dashboard shows** real-time status
- [ ] **Network simulator runs** and adds activity
- [ ] **Complete P2P workflow** demonstrated
- [ ] **No simulation-only features** (everything works)

---

## 🌟 **Conclusion**

The XMBL P2P Network is designed to be a **truly functional system**, not just a simulation. 

**If you see:**
- ✅ Real progress bars and status updates
- ✅ Unique CIDs and task IDs generated
- ✅ Actual file processing and storage
- ✅ Real-time compute task execution
- ✅ Working CLI integration
- ✅ Live monitoring data

**Then the system is working correctly!**

The simulator adds additional network activity on top of your webapp and CLI operations, creating a rich, dynamic P2P environment.

---

**🎯 GOAL: Verify that uploads actually work and the system is functional, not simulated!**
