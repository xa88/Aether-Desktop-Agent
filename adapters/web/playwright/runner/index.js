const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

// node runner/index.js '<json_payload>'
async function main() {
    const payloadStr = process.argv[2];
    if (!payloadStr) {
        console.error("Missing payload argument");
        process.exit(1);
    }

    let req;
    try {
        req = JSON.parse(payloadStr);
    } catch (e) {
        console.error("Invalid JSON payload");
        process.exit(1);
    }

    const browser = await chromium.launch({ headless: true });
    const context = await browser.newContext();

    // Enable tracing
    await context.tracing.start({ screenshots: true, snapshots: true });

    const page = await context.newPage();

    try {
        const action = req.action;
        const args = req.args;
        let result = {};

        if (action === 'navigate') {
            await page.goto(args.url);
            result.url = page.url();
        } else if (action === 'click') {
            const locator = page.locator(args.selector);
            await locator.click();
            result.clicked = true;
        } else if (action === 'fill') {
            const locator = page.locator(args.selector);
            await locator.fill(args.value);
            result.filled = true;
        } else if (action === 'expect_text') {
            const text = await page.locator(args.selector).textContent();
            if (!text.includes(args.text)) {
                throw new Error(`Expected text '${args.text}' not found, got '${text}'`);
            }
            result.verified = true;
        } else {
            throw new Error(`Unknown playwright action: ${action}`);
        }

        console.log(JSON.stringify({ success: true, output: result }));
    } catch (err) {
        // Attempt dump trace on fail
        const runId = req.run_id || 'unknown';
        const tracePath = path.join(process.cwd(), 'runs', `${runId}-trace.zip`);
        await context.tracing.stop({ path: tracePath });

        console.log(JSON.stringify({
            success: false,
            error: err.message,
            trace: tracePath
        }));
    } finally {
        await browser.close();
    }
}

main().catch(console.error);
