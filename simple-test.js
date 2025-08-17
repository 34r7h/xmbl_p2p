const puppeteer = require('puppeteer');

async function simpleTest() {
    console.log('🧪 Simple Test of localStorage Persistence...');
    
    const browser = await puppeteer.launch({ 
        headless: false, 
        slowMo: 1000
    });
    
    try {
        const page = await browser.newPage();
        
        console.log('📱 Testing localStorage persistence...');
        await page.goto('http://localhost:3002/test-persistence.html');
        
        // Wait for page to load
        await page.waitForSelector('#output', { timeout: 10000 });
        console.log('✅ Test page loaded');
        
        // Take initial screenshot
        await page.screenshot({ path: 'test-initial.png' });
        console.log('📸 Initial screenshot saved');
        
        // Check initial state
        const initialOutput = await page.$eval('#output', el => el.textContent);
        console.log('📊 Initial state:', initialOutput.includes('empty') ? 'Empty' : 'Has data');
        
        // Create a test file
        console.log('📁 Creating test file...');
        await page.evaluate(() => {
            const testFile = new File(['test content'], 'test.txt', { type: 'text/plain' });
            const dataTransfer = new DataTransfer();
            dataTransfer.items.add(testFile);
            document.getElementById('testFile').files = dataTransfer.files;
        });
        
        // Click test upload
        console.log('📤 Testing upload...');
        await page.click('button[onclick="testUpload()"]');
        
        // Wait for alert and dismiss it
        await page.waitForFunction(() => {
            return window.alert !== undefined;
        }, { timeout: 5000 });
        
        // Handle alert
        page.on('dialog', dialog => {
            console.log('📢 Alert:', dialog.message());
            dialog.dismiss();
        });
        
        // Wait a moment for the upload to complete
        await page.waitForTimeout(2000);
        
        // Check if file was added
        const afterUploadOutput = await page.$eval('#output', el => el.textContent);
        console.log('📊 After upload:', afterUploadOutput.includes('test.txt') ? 'File found' : 'File not found');
        
        // Take screenshot after upload
        await page.screenshot({ path: 'test-after-upload.png' });
        console.log('📸 Post-upload screenshot saved');
        
        // Check localStorage
        const localStorageData = await page.evaluate(() => {
            return localStorage.getItem('xmbl_stored_files');
        });
        
        if (localStorageData) {
            console.log('✅ localStorage contains data:', localStorageData);
        } else {
            console.log('❌ localStorage is empty');
        }
        
        // Refresh the page to test persistence
        console.log('🔄 Refreshing page to test persistence...');
        await page.reload();
        
        // Wait for page to load
        await page.waitForSelector('#output', { timeout: 10000 });
        
        // Check if data persisted
        const afterRefreshOutput = await page.$eval('#output', el => el.textContent);
        console.log('📊 After refresh:', afterRefreshOutput.includes('test.txt') ? 'File persisted' : 'File lost');
        
        // Take final screenshot
        await page.screenshot({ path: 'test-after-refresh.png' });
        console.log('📸 Post-refresh screenshot saved');
        
        // Final localStorage check
        const finalLocalStorage = await page.evaluate(() => {
            return localStorage.getItem('xmbl_stored_files');
        });
        
        if (finalLocalStorage && finalLocalStorage.includes('test.txt')) {
            console.log('🎉 SUCCESS: Data persisted after refresh!');
        } else {
            console.log('❌ FAILED: Data not persisted after refresh');
        }
        
    } catch (error) {
        console.error('❌ TEST FAILED:', error.message);
        await page.screenshot({ path: 'test-error.png' });
        throw error;
    } finally {
        await browser.close();
    }
}

simpleTest().catch(console.error);
