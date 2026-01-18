# Fixing "App is Damaged" Error on macOS

## The Problem

When downloading Axiom from GitHub releases, macOS Gatekeeper may show:
- "Axiom.app is damaged and can't be opened"
- "You should move it to the Trash"

This happens because the app is **not code-signed** (requires Apple Developer account).

---

## Solution 1: Remove Quarantine Attribute (Recommended)

After downloading and extracting the app:

```bash
# Remove the quarantine attribute
xattr -cr /Applications/Axiom.app

# Or if you extracted it elsewhere:
xattr -cr ~/Downloads/Axiom.app
```

Then try opening the app again.

---

## Solution 2: Allow in System Settings

1. Try to open Axiom.app (it will fail)
2. Go to **System Settings** → **Privacy & Security**
3. Scroll down to find "Axiom.app was blocked"
4. Click **"Open Anyway"**
5. Confirm by clicking **"Open"**

---

## Solution 3: Right-Click Method

1. Right-click (or Control+click) on Axiom.app
2. Select **"Open"** from the menu
3. Click **"Open"** in the security dialog
4. The app will now launch

---

## Solution 4: Disable Gatekeeper Temporarily (Advanced)

**Warning**: This reduces security. Re-enable after installation.

```bash
# Disable Gatekeeper
sudo spctl --master-disable

# Install and open Axiom.app

# Re-enable Gatekeeper
sudo spctl --master-enable
```

---

## For DMG Users

If using the DMG installer:

```bash
# After mounting the DMG and copying to Applications
xattr -cr /Applications/Axiom.app
```

---

## Why This Happens

- Axiom is not code-signed with an Apple Developer certificate
- macOS Gatekeeper blocks unsigned apps by default
- The quarantine attribute is added to downloaded files
- Removing the attribute tells macOS you trust the app

---

## Future Solution

To avoid this in future releases, the app needs to be:
1. **Code-signed** with an Apple Developer certificate ($99/year)
2. **Notarized** by Apple (automated process after signing)

This would allow the app to open without any warnings.

---

## Verification

After applying the fix, verify the app opens:

```bash
# Check if quarantine attribute is removed
xattr -l /Applications/Axiom.app
# Should show no com.apple.quarantine attribute

# Try opening
open /Applications/Axiom.app
```

---

## Still Having Issues?

If the app still won't open:

1. Check macOS version (requires 11.0+)
2. Verify you're on Apple Silicon (ARM64)
3. Check Console.app for error messages
4. Open an issue: https://github.com/hawklogic/axiom/issues

---

## Security Note

The `xattr -cr` command is safe and only removes the quarantine flag. It does not disable any system security features. You're explicitly telling macOS that you trust this specific app.
