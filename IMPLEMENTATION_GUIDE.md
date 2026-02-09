# Birdie Notification Listener - Implementation Guide

## Overview

This guide documents the cross-platform notification listener system for Birdie, a Tauri-based desktop application. The system is designed to capture system notifications from various applications in real-time.

## Project Status

✅ **Completed:**
- Project structure and modular architecture
- Cargo.toml dependencies configured
- Tauri app integration with background listener task
- Platform-specific module stubs with detailed TODO comments
- Full project compilation without errors

⏳ **Next Phase:**
- Complete Windows WinRT async/await implementation
- Complete macOS Objective-C interop implementation
- Complete Linux D-Bus listener implementation
- Frontend TypeScript event listener
- Real-world testing with Teams, Slack, Google Chat, etc.

---

## Architecture Overview

```
src-tauri/
├── src/
│   ├── lib.rs                 # Main entry point with Tauri setup
│   ├── main.rs                # Binary entry (unchanged)
│   └── notifications/
│       ├── mod.rs            # Shared types and public API
│       ├── windows.rs        # Windows UserNotificationListener (WinRT)
│       ├── macos.rs          # macOS NSDistributedNotificationCenter
│       └── linux.rs          # Linux D-Bus org.freedesktop.Notifications
├── Cargo.toml                # Dependencies with platform-specific features
```

### Data Flow

```
System App → Platform API → Notification Listener → Tauri Event → Frontend JavaScript
  Teams        UserNotificationListener    windows.rs      emit()     React Component
  Slack        NSDistributedNotificationCenter macos.rs
  Outlook      org.freedesktop.Notifications linux.rs
```

---

## Shared Types

### NotificationEvent
```rust
pub struct NotificationEvent {
    pub app_name: String,           // "Microsoft Teams", "Slack", etc.
    pub sender: String,             // Email, username, or contact name
    pub message: String,            // Notification message body
    pub timestamp: String,          // RFC3339 formatted timestamp
    pub app_icon: Option<String>,   // Base64 or path to app icon
}
```

### NotificationError
```rust
pub struct NotificationError {
    pub platform: String,           // "Windows", "macOS", or "Linux"
    pub error: String,              // Human-readable error description
    pub suggestion: String,         // How to fix the problem
}
```

---

## Windows Implementation (`notifications/windows.rs`)

### Technology Stack
- **API:** Windows Runtime (WinRT) `UserNotificationListener`
- **Crate:** `windows` v0.58
- **Minimum OS:** Windows 10 Build 14393 or Windows 11

### Key Components

1. **Permission System**
   ```rust
   UserNotificationListener::Current()?
   listener.RequestAccessAsync()? // Returns UserNotificationListenerAccessStatus
   // User sees: "This app would like to access your notifications"
   ```

2. **Notification Retrieval**
   ```rust
   listener.GetNotificationsAsync(NotificationKinds::Toast)?
   // Returns IVectorView<UserNotification>
   ```

3. **Real-time Monitoring**
   ```rust
   listener.NotificationChanged += callback
   // Receives UserNotificationChangedEventArgs with ChangeKind (Added, Removed, Updated)
   ```

### Implementation Checklist

- [ ] COM runtime initialization
- [ ] Request user permission with `RequestAccessAsync()`
- [ ] Handle permission denied scenario gracefully
- [ ] Fetch initial notifications with `GetNotificationsAsync()`
- [ ] Parse XML notification content (using `Data_Xml_Dom::XmlDocument`)
- [ ] Register `NotificationChanged` event handler
- [ ] Create notification event callbacks
- [ ] Emit `notification-received` event to frontend
- [ ] Handle Windows-specific errors and edge cases

### Example Windows Notification Structure
```
<toast>
  <visual>
    <binding template="ToastText02">
      <text id="1">Microsoft Teams</text>
      <text id="2">John Smith: Great work on the project!</text>
    </binding>
  </visual>
</toast>
```

### Notes
- Works best with MSIX-packaged applications but can work unpackaged
- Some older UWP apps may not report notifications properly
- Desktop (Win32) apps using modern toast APIs will be captured
- Requires Windows notification settings to enable the app

---

## macOS Implementation (`notifications/macos.rs`)

### Technology Stack
- **API:** `NSDistributedNotificationCenter`
- **Framework:** Cocoa (Foundation)
- **Crates:** `objc`, `objc-foundation`, `objc_id`, `cocoa`
- **Minimum OS:** macOS 10.14+

### Key Components

1. **Notification Center Access**
   ```objc
   NSDistributedNotificationCenter *center = [NSDistributedNotificationCenter defaultCenter];
   ```

2. **Observer Registration**
   ```objc
   [center addObserverForName:notification_name
                      object:nil
                       queue:nil
                   usingBlock:^(NSNotification *note) { ... }];
   ```

3. **Common Notification Names**
   - `com.microsoft.teams.notification`
   - `com.slack.Slack.notification`
   - `com.google.chat.notification`
   - `com.apple.mail.notification`

### Implementation Checklist

- [ ] Initialize NSAutorelease pool for thread safety
- [ ] Get NSDistributedNotificationCenter::defaultCenter()
- [ ] Register observers for known app notification names
- [ ] Create block callbacks for notification reception
- [ ] Parse NSNotification userInfo dictionary
- [ ] Extract app_name, sender, message from userInfo
- [ ] Handle different app notification formats
- [ ] Emit `notification-received` event
- [ ] Manage observer lifecycle (dealloc/cleanup)

### Example NSNotification Structure
```
NSNotification {
  name: "com.slack.Slack.notification",
  userInfo: {
    "title": "Channel: #general",
    "body": "John Smith: Let's sync up tomorrow",
    "icon": "/path/to/slack.icns"
  }
}
```

### Notes
- No explicit permission request needed
- App sandbox may require exceptions to receive distributed notifications
- Different apps post with varying notification formats
- Some private APIs may require App Sandbox disabling
- Main thread operations may be required for some Cocoa APIs

---

## Linux Implementation (`notifications/linux.rs`)

### Technology Stack
- **API:** org.freedesktop.Notifications (D-Bus)
- **Crate:** `zbus` v4.0
- **Minimum:** Ubuntu 20.04+, GNOME, KDE Plasma

### Key Components

1. **D-Bus Service Registration**
   ```rust
   let connection = Connection::session().await?;
   connection.request_name("org.freedesktop.Notifications").await?;
   ```

2. **Interface Implementation**
   - `GetCapabilities()` → Vec of capability strings
   - `Notify(app_name, replaces_id, app_icon, summary, body, actions, hints, timeout)` → notification_id
   - `CloseNotification(id)` → ()
   - Signals: `NotificationClosed(id, reason)`, `ActionInvoked(id, action_key)`

3. **Incoming Notifications**
   ```rust
   pub async fn notify(
       app_name: String,
       replaces_id: u32,
       app_icon: String,
       summary: String,
       body: String,
       actions: Vec<String>,
       hints: Dict<String, Variant>,
       expire_timeout: i32,
   ) -> u32
   ```

### Implementation Checklist

- [ ] Connect to D-Bus session bus
- [ ] Create zbus server with Notifications interface
- [ ] Implement GetCapabilities() method
- [ ] Implement Notify() method to receive notifications
- [ ] Parse notification parameters into NotificationEvent
- [ ] Implement CloseNotification() method
- [ ] Handle hints dictionary for app icon extraction
- [ ] Emit `notification-received` event to frontend
- [ ] Implement NotificationClosed and ActionInvoked signals

### Example D-Bus Notify Call
```
Method: org.freedesktop.Notifications.Notify
Arguments:
  - app_name: "Slack"
  - replaces_id: 0
  - app_icon: "slack"
  - summary: "John Smith"
  - body: "Let's sync up tomorrow"
  - actions: ["default", "Open"]
  - hints: {"urgency": 1, "x-canonical-append": true}
  - expire_timeout: 5000
Returns: 1234 (notification ID)
```

### Notes
- Multiple notification servers can coexist on D-Bus
- Some desktop environments prefer specific notification daemons
- Hints dictionary contains app-specific metadata
- Actions allow user interactions with notifications
- Proper error handling for D-Bus communication failures

---

## Tauri Integration

### Setup Hook (`lib.rs`)

The notification listener is started in the Tauri app setup hook:

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();

            // Spawn notification listener in background
            tauri::async_runtime::spawn(async move {
                notifications::start_notification_listener(handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Key Points
- ✅ Uses `app.handle()` not `app.app_handle()`
- ✅ Clones handle for async context
- ✅ Uses `tauri::async_runtime::spawn()` for non-blocking execution
- ✅ Listener runs indefinitely in background
- ✅ Spawned before app window is created

---

## Frontend Integration

### TypeScript Event Listener

```typescript
import { listen } from '@tauri-apps/api/event';

interface NotificationEvent {
    app_name: string;
    sender: string;
    message: string;
    timestamp: string;
    app_icon?: string;
}

export async function setupNotificationListener() {
    const unlisten = await listen<NotificationEvent>(
        'notification-received',
        (event) => {
            console.log('New notification:', event.payload);
            // Update UI, store in state, add to notification list, etc.
        }
    );

    return unlisten;
}
```

### Error Handling

```typescript
export async function setupErrorListener() {
    const unlisten = await listen(
        'notification-error',
        (event) => {
            const { platform, error, suggestion } = event.payload;
            console.error(`${platform} error: ${error}`);
            console.info(`Fix: ${suggestion}`);
        }
    );

    return unlisten;
}
```

---

## Testing Strategy

### Windows Testing
1. Build: `cd src-tauri && cargo build`
2. Run: `cargo tauri dev`
3. Trigger notifications from Teams/Slack/Outlook
4. Verify console output: `📬 Notification captured: Teams`
5. Check frontend receives `notification-received` event
6. Test permission denial scenario

### macOS Testing
1. Build on macOS machine
2. Run: `cargo tauri dev`
3. Send test notifications from Slack/Messages
4. Verify console output
5. Test app filtering (observe specific apps)
6. Check sandbox permissions if needed

### Linux Testing
1. Build on Linux (Ubuntu 20.04+)
2. Run: `cargo tauri dev`
3. Send notifications from test app
4. Verify D-Bus interface registration
5. Test with GNOME/KDE notification daemon
6. Check for D-Bus permission errors

### Cross-Platform Testing
1. Ensure code compiles on all platforms
2. Test conditional compilation accuracy
3. Verify no platform-specific code leaks
4. Test API consistency across platforms

---

## Debugging & Troubleshooting

### Windows Issues

**Permission Denied**
- Solution: Enable in Settings → Privacy & Security → Notifications
- Also check notification panel for app permissions

**WinRT API Errors**
- Verify Windows 10 Build 14393+ or Windows 11
- Check COM runtime initialization
- Ensure STA (Single-Threaded Apartment) for UI operations

**No Notifications Captured**
- Verify target app uses standard Toast notifications
- Check app isn't using custom notification system
- Confirm listener has permission

### macOS Issues

**Notifications Not Received**
- Check app sandbox configuration
- Verify notification names are correct for target app
- May need to disable sandbox for distributed notifications

**Objective-C Memory Issues**
- Ensure proper autorelease pool management
- Check for retain/release cycles in block callbacks
- Use weak references where needed

**Observer Not Triggering**
- Verify block callback syntax
- Check notification names against documentation
- Test with known app (e.g., Slack)

### Linux Issues

**D-Bus Connection Failed**
- Ensure `dbus` system service is running
- Check `DBUS_SESSION_BUS_ADDRESS` environment variable
- Verify session bus accessibility

**Service Not Registered**
- Check D-Bus name conflicts
- Verify zbus interface implementation
- Review D-Bus permission policies

---

## Future Enhancements

1. **App Filtering**
   - Configuration to select which apps to monitor
   - Whitelist/blacklist system

2. **Notification Actions**
   - Capture action buttons (Reply, Dismiss)
   - Send action callbacks back to source app

3. **Media & Images**
   - Extract notification images
   - Cache app icons

4. **Persistence**
   - Store notifications in local database
   - Query historical notifications

5. **Sync**
   - Cross-device notification sync
   - Cloud backup of important notifications

6. **AI Integration**
   - Smart filtering and categorization
   - "Jarvis" mode for intelligent response

---

## References

### Windows
- [UserNotificationListener MSDN](https://learn.microsoft.com/en-us/uwp/api/windows.ui.notifications.management.usernotificationlistener)
- [Toast Notifications Overview](https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/adaptive-interactive-toasts)
- [windows-rs GitHub](https://github.com/microsoft/windows-rs)
- [WinRT COM Interop](https://docs.rs/windows/latest/windows/)

### macOS
- [NSDistributedNotificationCenter Apple Docs](https://developer.apple.com/documentation/foundation/nsdistributednotificationcenter)
- [Cocoa Notifications](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Notifications/)
- [objc Rust Crate](https://crates.io/crates/objc)
- [objc-foundation](https://crates.io/crates/objc-foundation)

### Linux
- [org.freedesktop.Notifications Specification](https://specifications.freedesktop.org/notification-spec/latest/)
- [D-Bus Specification](https://dbus.freedesktop.org/doc/dbus-specification.html)
- [zbus Documentation](https://docs.rs/zbus/latest/zbus/)
- [FreeDesktop.org Notifications](https://www.freedesktop.org/wiki/Specifications/NotificationAPISpec/)

### Tauri
- [Tauri Events Documentation](https://tauri.app/develop/calling-frontend/)
- [Tauri Commands](https://tauri.app/develop/calling-rust/)
- [Tauri Plugins](https://tauri.app/develop/plugins/)
- [Async Runtime](https://docs.rs/tauri/latest/tauri/)

---

## File Structure Summary

```
src-tauri/
├── Cargo.toml
│   ├── [dependencies]
│   │   ├── tauri = "2"
│   │   ├── tokio = "1" (with "full" features)
│   │   ├── chrono = "0.4"
│   │   ├── serde/serde_json
│   │   └── tauri-plugin-opener
│   ├── [target.'cfg(target_os = "windows")'.dependencies]
│   │   └── windows = "0.58"
│   ├── [target.'cfg(target_os = "macos")'.dependencies]
│   │   ├── objc = "0.2"
│   │   ├── objc-foundation = "0.1"
│   │   ├── objc_id = "0.1"
│   │   └── cocoa = "0.25"
│   └── [target.'cfg(target_os = "linux")'.dependencies]
│       └── zbus = "4.0"
├── src/
│   ├── lib.rs (modified - setup hook added)
│   ├── main.rs (unchanged)
│   └── notifications/
│       ├── mod.rs (public API, shared types)
│       ├── windows.rs (WinRT listener - NEEDS COMPLETION)
│       ├── macos.rs (NSDistributedNotificationCenter - NEEDS COMPLETION)
│       └── linux.rs (D-Bus listener - NEEDS COMPLETION)
```

---

## Next Steps

1. **Complete Windows Implementation**
   - Implement WinRT async/await conversion
   - Add XML parsing for notification content
   - Handle event callbacks

2. **Complete macOS Implementation**
   - Implement Objective-C block callbacks
   - Parse NSNotification userInfo
   - Handle observer lifecycle

3. **Complete Linux Implementation**
   - Implement D-Bus service registration
   - Add Notify method handler
   - Parse D-Bus hints dictionary

4. **Frontend Development**
   - Add React components for notification display
   - Implement notification list/history
   - Add filtering and search

5. **Testing & Documentation**
   - Cross-platform testing
   - User documentation
   - API documentation for extensions

---

**Status:** Implementation skeleton complete, platform-specific details pending.
**Last Updated:** 2026-02-08
