# ADA Release Checklist

Before pushing a compiled Aether Desktop Agent mapping to production deployment fleets, operations personnel must iterate this standard distribution pipeline fully ensuring platform integrity validation constraints pass securely.

### Development Matrix
- Verify that `profile.yaml` statically points towards Internal Models.
- Test that all Local sandbox Docker hooks map cleanly on unmodified Host systems.

### OSX Apple Notarization Pipeline
Electron Builder handles `.dmg` structure cleanly, but the raw binary payloads must negotiate explicitly around Gatekeeper constraints.
1. Obtain the `Developer ID Application` signing keys natively inside your enterprise KeyChain.
2. Configure `.env`:
   ```env
   APPLEID=it@example.com
   APPLEIDPASS=xxxx-xxxx-xxxx
   TEAMID=YOUR_TEAM
   ```
3. Invoke: `npm run build:mac`. The build process signs and submits the bundle directly towards the macOS developer APIs. Wait natively for the notarization digest tick.

### Windows EV Code Signing Hooks 
If deploying to high-security offline targets avoiding Windows SmartScreen triggers:
1. Slot the hardware Extended Validation USB tokens directly into the isolated Build Server.
2. In `package.json -> build -> win`, point the `certificateSubjectName` explicitly toward the EV parameter string matching the USB hardware mapping.
3. Validate `.msi`/`.exe` generated structurally in `./dist/` maps to `signtool.exe verify /pa` successfully.

### Linux AppImage/DEB
No hard Code-signing constraints limit Linux targets currently. `npm run build:linux` drops isolated binaries. Ensure the CI orchestrator writes explicit `sha256sum` digest arrays across all artifacts before publishing generically towards S3 or Github Release assets.
