const puppeteer = require('puppeteer');

async function takeScreenshot() {
    console.log('🚀 Starting XMBL P2P Swarm Dashboard Screenshot Test...');
    
    const browser = await puppeteer.launch({
        headless: false,
        defaultViewport: { width: 1200, height: 800 }
    });
    
    try {
        const page = await browser.newPage();
        
        console.log('📱 Navigating to dashboard...');
        await page.goto('http://localhost:3100/functional-client.html', { 
            waitUntil: 'networkidle0',
            timeout: 30000 
        });
        
        console.log('⏳ Waiting for dashboard to load...');
        await page.waitForTimeout(5000);
        
        console.log('📸 Taking screenshot...');
        await page.screenshot({ 
            path: 'xmbl-dashboard-status.png',
            fullPage: true 
        });
        
        console.log('🔍 Checking network status text...');
        const networkStatusText = await page.$eval('#networkStatusText', el => el.textContent);
        console.log(`Network Status: "${networkStatusText}"`);
        
        console.log('🔍 Checking if status indicator is online...');
        const isOnline = await page.$eval('#networkStatus', el => !el.classList.contains('offline'));
        console.log(`Status Indicator Online: ${isOnline}`);
        
        console.log('✅ Screenshot saved as xmbl-dashboard-status.png');
        console.log(`🎯 Dashboard shows: ${networkStatusText}`);
        
        if (networkStatusText.includes('Online') && networkStatusText.includes('P2P swarm')) {
            console.log('🎉 SUCCESS: Dashboard is showing online P2P swarm status!');
        } else {
            console.log('❌ FAILED: Dashboard is not showing online P2P swarm status');
        }
        
    } catch (error) {
        console.error('❌ Error taking screenshot:', error.message);
    } finally {
        await browser.close();
    }
}

takeScreenshot();

