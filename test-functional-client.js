const puppeteer = require('puppeteer');

async function testFunctionalClient() {
    console.log('🧪 Testing XMBL Functional Client...');
    
    const browser = await puppeteer.launch({ 
        headless: false, 
        slowMo: 1000,
        args: ['--no-sandbox', '--disable-setuid-sandbox']
    });
    
    try {
        const page = await browser.newPage();
        
        // Set viewport
        await page.setViewport({ width: 1200, height: 800 });
        
        console.log('📱 Navigating to functional client...');
        await page.goto('http://localhost:3000/functional-client.html', { 
            waitUntil: 'networkidle2',
            timeout: 30000 
        });
        
        // Wait for page to load
        await page.waitForSelector('.container', { timeout: 10000 });
        console.log('✅ Page loaded successfully');
        
        // Take initial screenshot
        await page.screenshot({ path: 'test-initial.png', fullPage: true });
        console.log('📸 Initial screenshot saved');
        
        // Test 1: Check if file input exists
        const fileInput = await page.$('#fileInput');
        if (!fileInput) {
            throw new Error('❌ File input not found');
        }
        console.log('✅ File input found');
        
        // Test 2: Check if redundancy selector exists
        const redundancySelect = await page.$('#redundancy');
        if (!redundancySelect) {
            throw new Error('❌ Redundancy selector not found');
        }
        console.log('✅ Redundancy selector found');
        
        // Test 3: Check if store button exists
        const storeButton = await page.$('button[onclick="storeFile()"]');
        if (!storeButton) {
            throw new Error('❌ Store button not found');
        }
        console.log('✅ Store button found');
        
        // Test 4: Check initial state (should be empty)
        const initialFileList = await page.$eval('#fileList', el => el.textContent);
        if (!initialFileList.includes('No files stored yet')) {
            console.log('⚠️ Initial state not as expected:', initialFileList);
        } else {
            console.log('✅ Initial state correct - no files stored');
        }
        
        // Test 5: Create a test file and upload it
        console.log('📁 Creating test file...');
        await page.evaluate(() => {
            // Create a test file
            const testFile = new File(['This is a test file content'], 'test-file.txt', { type: 'text/plain' });
            
            // Create a new DataTransfer object
            const dataTransfer = new DataTransfer();
            dataTransfer.items.add(testFile);
            
            // Set the file input value
            const fileInput = document.getElementById('fileInput');
            fileInput.files = dataTransfer.files;
            
            // Trigger change event
            fileInput.dispatchEvent(new Event('change', { bubbles: true }));
        });
        
        console.log('📤 Uploading test file...');
        await page.click('button[onclick="storeFile()"]');
        
        // Wait for upload to start
        await page.waitForFunction(() => {
            const fileList = document.getElementById('fileList');
            return fileList.textContent.includes('Processing...');
        }, { timeout: 10000 });
        
        console.log('✅ File upload started');
        
        // Wait for upload to complete
        await page.waitForFunction(() => {
            const fileList = document.getElementById('fileList');
            return fileList.textContent.includes('File stored successfully');
        }, { timeout: 30000 });
        
        console.log('✅ File upload completed');
        
        // Take screenshot after upload
        await page.screenshot({ path: 'test-after-upload.png', fullPage: true });
        console.log('📸 Post-upload screenshot saved');
        
        // Test 6: Check if file is in the list
        const fileListContent = await page.$eval('#fileList', el => el.textContent);
        if (!fileListContent.includes('test-file.txt')) {
            throw new Error('❌ Uploaded file not found in list');
        }
        console.log('✅ Uploaded file found in list');
        
        // Test 7: Check localStorage persistence
        const localStorageData = await page.evaluate(() => {
            return {
                files: localStorage.getItem('xmbl_stored_files'),
                tasks: localStorage.getItem('xmbl_compute_tasks')
            };
        });
        
        if (!localStorageData.files) {
            throw new Error('❌ No files data in localStorage');
        }
        
        const filesData = JSON.parse(localStorageData.files);
        if (filesData.length === 0) {
            throw new Error('❌ Files array is empty in localStorage');
        }
        
        console.log('✅ localStorage contains files data:', filesData.length, 'files');
        
        // Test 8: Refresh the page to test persistence
        console.log('🔄 Refreshing page to test persistence...');
        await page.reload({ waitUntil: 'networkidle2' });
        
        // Wait for page to load
        await page.waitForSelector('.container', { timeout: 10000 });
        
        // Check if file is still there after refresh
        const fileListAfterRefresh = await page.$eval('#fileList', el => el.textContent);
        if (!fileListAfterRefresh.includes('test-file.txt')) {
            throw new Error('❌ File not persisted after refresh');
        }
        console.log('✅ File persisted after refresh');
        
        // Take final screenshot
        await page.screenshot({ path: 'test-after-refresh.png', fullPage: true });
        console.log('📸 Post-refresh screenshot saved');
        
        // Test 9: Check localStorage after refresh
        const localStorageAfterRefresh = await page.evaluate(() => {
            return {
                files: localStorage.getItem('xmbl_stored_files'),
                tasks: localStorage.getItem('xmbl_compute_tasks')
            };
        });
        
        if (!localStorageAfterRefresh.files) {
            throw new Error('❌ localStorage data lost after refresh');
        }
        
        const filesAfterRefresh = JSON.parse(localStorageAfterRefresh.files);
        if (filesAfterRefresh.length === 0) {
            throw new Error('❌ Files array empty after refresh');
        }
        
        console.log('✅ localStorage data intact after refresh:', filesAfterRefresh.length, 'files');
        
        // Test 10: Test compute task submission
        console.log('⚡ Testing compute task submission...');
        
        // Create a test WASM file
        await page.evaluate(() => {
            const testWasmFile = new File(['mock wasm content'], 'test-task.wasm', { type: 'application/wasm' });
            
            const dataTransfer = new DataTransfer();
            dataTransfer.items.add(testWasmFile);
            
            const wasmInput = document.getElementById('wasmFile');
            wasmInput.files = dataTransfer.files;
            wasmInput.dispatchEvent(new Event('change', { bubbles: true }));
            
            // Set input data
            const inputData = document.getElementById('inputData');
            inputData.value = '{"test": "data", "operation": "sum"}';
        });
        
        // Submit compute task
        await page.click('button[onclick="submitComputeTask()"]');
        
        // Wait for task to start
        await page.waitForFunction(() => {
            const taskList = document.getElementById('taskList');
            return taskList.textContent.includes('Processing...');
        }, { timeout: 10000 });
        
        console.log('✅ Compute task started');
        
        // Wait for task to complete
        await page.waitForFunction(() => {
            const taskList = document.getElementById('taskList');
            return taskList.textContent.includes('Task completed successfully');
        }, { timeout: 30000 });
        
        console.log('✅ Compute task completed');
        
        // Test 11: Check if task is in localStorage
        const finalLocalStorage = await page.evaluate(() => {
            return {
                files: localStorage.getItem('xmbl_stored_files'),
                tasks: localStorage.getItem('xmbl_compute_tasks')
            };
        });
        
        if (!finalLocalStorage.tasks) {
            throw new Error('❌ No tasks data in localStorage');
        }
        
        const tasksData = JSON.parse(finalLocalStorage.tasks);
        if (tasksData.length === 0) {
            throw new Error('❌ Tasks array is empty in localStorage');
        }
        
        console.log('✅ localStorage contains tasks data:', tasksData.length, 'tasks');
        
        // Final screenshot
        await page.screenshot({ path: 'test-final.png', fullPage: true });
        console.log('📸 Final screenshot saved');
        
        console.log('\n🎉 ALL TESTS PASSED! The functional client is working correctly.');
        console.log('📊 Test Summary:');
        console.log('   ✅ File upload functionality');
        console.log('   ✅ Data persistence in localStorage');
        console.log('   ✅ Page refresh persistence');
        console.log('   ✅ Compute task submission');
        console.log('   ✅ Task persistence in localStorage');
        
    } catch (error) {
        console.error('❌ TEST FAILED:', error.message);
        await page.screenshot({ path: 'test-error.png', fullPage: true });
        console.log('📸 Error screenshot saved');
        throw error;
    } finally {
        await browser.close();
    }
}

// Run the test
testFunctionalClient().catch(console.error);
