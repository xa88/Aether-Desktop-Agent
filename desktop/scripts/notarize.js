/**
 * macOS Notarization Hook — invoked by electron-builder after signing.
 * This stub is a no-op on Windows/Linux; real notarization requires:
 *   - APPLEID, APPLEIDPASS, TEAMID environment variables
 *   - @electron/notarize installed in devDependencies
 * See docs/release_checklist.md for full macOS notarization instructions.
 */

exports.default = async function notarizeMacos(context) {
  const { electronPlatformName, appOutDir } = context;

  // Only notarize on macOS targets
  if (electronPlatformName !== 'darwin') {
    return;
  }

  // Check if credentials are present before attempting notarization
  if (!process.env.APPLEID || !process.env.APPLEIDPASS || !process.env.TEAMID) {
    console.warn('[notarize] Skipping: APPLEID / APPLEIDPASS / TEAMID not set. Ensure these are configured in your CI environment for production releases.');
    return;
  }

  // Production: uncomment and configure the notarize call below
  // const { notarize } = require('@electron/notarize');
  // await notarize({
  //   appBundleId: 'ai.antigravity.aether-desktop-agent',
  //   appPath: `${appOutDir}/Aether Desktop Agent.app`,
  //   appleId: process.env.APPLEID,
  //   appleIdPassword: process.env.APPLEIDPASS,
  //   teamId: process.env.TEAMID,
  // });

  console.log('[notarize] macOS notarization hook complete (stub).');
};
