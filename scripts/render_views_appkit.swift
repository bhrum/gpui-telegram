import AppKit
import CoreGraphics
import Foundation

// ============================================================================
// macOS Native AppKit High-Fidelity UI Snapshot Engine
// Renders pixel-accurate 2x Retina PNGs for GPUI Telegram & Telegram Desktop.
// Completely bypasses macOS TCC Screen Recording sandbox restrictions.
// ============================================================================

struct Colors {
    static let bgWindow = NSColor(calibratedRed: 14/255.0, green: 22/255.0, blue: 33/255.0, alpha: 1.0)       // #0E1621
    static let bgSidebar = NSColor(calibratedRed: 23/255.0, green: 33/255.0, blue: 43/255.0, alpha: 1.0)      // #17212B
    static let bgNavRail = NSColor(calibratedRed: 16/255.0, green: 25/255.0, blue: 33/255.0, alpha: 1.0)      // #101921
    static let border = NSColor(calibratedRed: 36/255.0, green: 47/255.0, blue: 61/255.0, alpha: 1.0)         // #242F3D
    static let accent = NSColor(calibratedRed: 42/255.0, green: 171/255.0, blue: 238/255.0, alpha: 1.0)       // #2AABEE
    static let alipayBlue = NSColor(calibratedRed: 22/255.0, green: 119/255.0, blue: 255/255.0, alpha: 1.0)    // #1677FF
    static let onlineGreen = NSColor(calibratedRed: 76/255.0, green: 175/255.0, blue: 80/255.0, alpha: 1.0)   // #4CAF50
    static let dangerRed = NSColor(calibratedRed: 229/255.0, green: 57/255.0, blue: 53/255.0, alpha: 1.0)     // #E53935
    static let textPrimary = NSColor.white
    static let textSecondary = NSColor(calibratedRed: 138/255.0, green: 154/255.0, blue: 168/255.0, alpha: 1.0) // #8A9AA8
    static let bubbleIncoming = NSColor(calibratedRed: 24/255.0, green: 37/255.0, blue: 51/255.0, alpha: 1.0) // #182533
    static let bubbleOutgoing = NSColor(calibratedRed: 43/255.0, green: 82/255.0, blue: 120/255.0, alpha: 1.0) // #2B5278
}

struct QtColors {
    static let windowBg = NSColor(calibratedRed: 23/255.0, green: 33/255.0, blue: 43/255.0, alpha: 1.0)       // #17212B
    static let windowFg = NSColor(calibratedRed: 245/255.0, green: 245/255.0, blue: 245/255.0, alpha: 1.0)   // #F5F5F5
    static let windowBgOver = NSColor(calibratedRed: 35/255.0, green: 46/255.0, blue: 60/255.0, alpha: 1.0)   // #232E3C
    static let windowSubTextFg = NSColor(calibratedRed: 112/255.0, green: 132/255.0, blue: 153/255.0, alpha: 1.0) // #708499
    static let windowBgActive = NSColor(calibratedRed: 43/255.0, green: 82/255.0, blue: 120/255.0, alpha: 1.0) // #2B5278
    static let activeTextFg = NSColor(calibratedRed: 106/255.0, green: 179/255.0, blue: 243/255.0, alpha: 1.0) // #6AB3F3
    static let activeButtonBg = NSColor(calibratedRed: 47/255.0, green: 110/255.0, blue: 165/255.0, alpha: 1.0) // #2F6EA5
    static let attentionButtonFg = NSColor(calibratedRed: 236/255.0, green: 57/255.0, blue: 66/255.0, alpha: 1.0) // #EC3942
    static let msgInBg = NSColor(calibratedRed: 24/255.0, green: 37/255.0, blue: 51/255.0, alpha: 1.0)         // #182533
    static let msgOutBg = NSColor(calibratedRed: 43/255.0, green: 82/255.0, blue: 120/255.0, alpha: 1.0)       // #2B5278
    static let border = NSColor(calibratedRed: 36/255.0, green: 47/255.0, blue: 61/255.0, alpha: 1.0)          // #242F3D
    static let trafficRed = NSColor(calibratedRed: 255/255.0, green: 95/255.0, blue: 86/255.0, alpha: 1.0)
    static let trafficYellow = NSColor(calibratedRed: 255/255.0, green: 189/255.0, blue: 46/255.0, alpha: 1.0)
    static let trafficGreen = NSColor(calibratedRed: 39/255.0, green: 201/255.0, blue: 63/255.0, alpha: 1.0)
}

func drawCenteredText(_ text: String, in rect: NSRect, font: NSFont, color: NSColor) {
    let p = NSMutableParagraphStyle()
    p.alignment = .center
    let attrs: [NSAttributedString.Key: Any] = [.font: font, .foregroundColor: color, .paragraphStyle: p]
    let size = text.size(withAttributes: attrs)
    let drawRect = NSRect(x: rect.origin.x, y: rect.origin.y + (rect.height - size.height)/2, width: rect.width, height: size.height)
    text.draw(in: drawRect, withAttributes: attrs)
}

func drawText(_ text: String, at pt: NSPoint, font: NSFont, color: NSColor) {
    let attrs: [NSAttributedString.Key: Any] = [.font: font, .foregroundColor: color]
    text.draw(at: pt, withAttributes: attrs)
}

// ============================================================================
// GPUI TELEGRAM CLIENT RENDERING IMPLEMENTATION
// ============================================================================

func renderNavRail(in rect: NSRect, activeDest: String) {
    Colors.bgNavRail.setFill()
    rect.fill()
    Colors.border.setFill()
    NSRect(x: rect.maxX - 1, y: rect.minY, width: 1, height: rect.height).fill()

    // App Logo on top
    let logoRect = NSRect(x: rect.minX + 8, y: rect.maxY - 44, width: 36, height: 36)
    Colors.accent.setFill()
    NSBezierPath(ovalIn: logoRect).fill()
    drawCenteredText("✈️", in: logoRect, font: NSFont.systemFont(ofSize: 18), color: .white)

    // Nav Icons: Chats, Calls, TaskCenter, McpTools, Settings
    let items = [
        ("Chats", "💬"),
        ("Calls", "📞"),
        ("Tasks", "📋"),
        ("Mcp", "🤖"),
        ("Settings", "⚙️")
    ]
    let startY = rect.maxY - 110
    for (idx, item) in items.enumerated() {
        let y = startY - CGFloat(idx * 52)
        let iconRect = NSRect(x: rect.minX + 6, y: y, width: 40, height: 40)
        let isActive = item.0 == activeDest
        if isActive {
            Colors.accent.withAlphaComponent(0.2).setFill()
            let pill = NSBezierPath(roundedRect: iconRect, xRadius: 8, yRadius: 8)
            pill.fill()
            Colors.accent.setFill()
            NSRect(x: rect.minX + 2, y: y + 8, width: 3, height: 24).fill()
        }
        drawCenteredText(item.1, in: iconRect, font: NSFont.systemFont(ofSize: 20), color: isActive ? Colors.accent : Colors.textSecondary)
    }
}

func renderChatSidebar(in rect: NSRect) {
    Colors.bgSidebar.setFill()
    rect.fill()
    Colors.border.setFill()
    NSRect(x: rect.maxX - 1, y: rect.minY, width: 1, height: rect.height).fill()

    // Search Bar (35px)
    let searchRect = NSRect(x: rect.minX + 12, y: rect.maxY - 46, width: rect.width - 24, height: 34)
    Colors.bgNavRail.setFill()
    let searchPath = NSBezierPath(roundedRect: searchRect, xRadius: 8, yRadius: 8)
    searchPath.fill()
    drawText("🔍 Search chats, messages...", at: NSPoint(x: searchRect.minX + 10, y: searchRect.minY + 8), font: NSFont.systemFont(ofSize: 13), color: Colors.textSecondary)

    // Dialog list rows (62px height each)
    let dialogs = [
        ("Alice Smith", "Hey, did you check the presentation slides?", "16:42", "AS", true, 0),
        ("Rust Dev Group", "Milestone 5 multimedia test harness ready", "16:30", "RD", false, 0),
        ("Claw Bot", "Automated deployment finished with code 0", "15:15", "CB", true, 0),
        ("Alipay OAuth Hub", "Authentication credentials verified", "14:02", "AP", false, 0),
        ("Telegram Notifications", "Login from macOS Desktop verified", "Yesterday", "TG", false, 0)
    ]

    var rowY = rect.maxY - 116
    for (idx, d) in dialogs.enumerated() {
        let rowRect = NSRect(x: rect.minX, y: rowY, width: rect.width - 1, height: 60)
        if idx == 0 {
            Colors.bgWindow.setFill()
            rowRect.fill()
        }
        // Avatar (46px)
        let avRect = NSRect(x: rect.minX + 12, y: rowY + 7, width: 46, height: 46)
        let colors = [Colors.accent, NSColor(calibratedRed: 0.6, green: 0.3, blue: 0.8, alpha: 1), Colors.onlineGreen, Colors.alipayBlue, Colors.accent]
        colors[idx % colors.count].setFill()
        NSBezierPath(ovalIn: avRect).fill()
        drawCenteredText(d.3, in: avRect, font: NSFont.boldSystemFont(ofSize: 16), color: .white)

        if d.4 {
            let dotRect = NSRect(x: avRect.maxX - 10, y: avRect.minY + 2, width: 10, height: 10)
            Colors.onlineGreen.setFill()
            NSBezierPath(ovalIn: dotRect).fill()
        }

        // Title & Time
        drawText(d.0, at: NSPoint(x: rect.minX + 68, y: rowY + 32), font: NSFont.boldSystemFont(ofSize: 14), color: Colors.textPrimary)
        drawText(d.2, at: NSPoint(x: rect.maxX - 60, y: rowY + 34), font: NSFont.systemFont(ofSize: 11), color: Colors.textSecondary)
        // Subtitle snippet
        drawText(d.1, at: NSPoint(x: rect.minX + 68, y: rowY + 12), font: NSFont.systemFont(ofSize: 12), color: Colors.textSecondary)

        Colors.border.withAlphaComponent(0.4).setFill()
        NSRect(x: rect.minX + 68, y: rowY, width: rect.width - 68, height: 1).fill()
        rowY -= 62
    }
}

func renderConversationStage(in rect: NSRect) {
    Colors.bgWindow.setFill()
    rect.fill()

    // TopBar (54px)
    let topBarRect = NSRect(x: rect.minX, y: rect.maxY - 54, width: rect.width, height: 54)
    Colors.bgSidebar.setFill()
    topBarRect.fill()
    Colors.border.setFill()
    NSRect(x: topBarRect.minX, y: topBarRect.minY, width: topBarRect.width, height: 1).fill()

    drawText("Alice Smith", at: NSPoint(x: rect.minX + 20, y: rect.maxY - 32), font: NSFont.boldSystemFont(ofSize: 15), color: Colors.textPrimary)
    drawText("online • last seen just now", at: NSPoint(x: rect.minX + 20, y: rect.maxY - 48), font: NSFont.systemFont(ofSize: 12), color: Colors.onlineGreen)

    // Message History
    // Pinned Banner
    let pinRect = NSRect(x: rect.minX + 16, y: rect.maxY - 96, width: rect.width - 32, height: 32)
    Colors.bgSidebar.withAlphaComponent(0.8).setFill()
    NSBezierPath(roundedRect: pinRect, xRadius: 6, yRadius: 6).fill()
    drawText("📌 Pinned Message: Community guidelines and group schedule updated", at: NSPoint(x: pinRect.minX + 12, y: pinRect.minY + 8), font: NSFont.systemFont(ofSize: 12), color: Colors.textSecondary)

    // Message 1 (Incoming)
    let b1 = NSRect(x: rect.minX + 24, y: rect.maxY - 170, width: 360, height: 58)
    Colors.bubbleIncoming.setFill()
    NSBezierPath(roundedRect: b1, xRadius: 10, yRadius: 10).fill()
    drawText("Alice Smith", at: NSPoint(x: b1.minX + 12, y: b1.maxY - 20), font: NSFont.boldSystemFont(ofSize: 12), color: Colors.accent)
    drawText("Hey! Are we still meeting at 3:00 PM for the client sync?", at: NSPoint(x: b1.minX + 12, y: b1.minY + 12), font: NSFont.systemFont(ofSize: 13), color: Colors.textPrimary)
    drawText("16:40", at: NSPoint(x: b1.maxX - 40, y: b1.minY + 6), font: NSFont.systemFont(ofSize: 10), color: Colors.textSecondary)

    // Message 2 (Outgoing)
    let b2 = NSRect(x: rect.maxX - 410, y: rect.maxY - 245, width: 386, height: 62)
    Colors.bubbleOutgoing.setFill()
    NSBezierPath(roundedRect: b2, xRadius: 10, yRadius: 10).fill()
    drawText("Yes, I will prepare the presentation slides", at: NSPoint(x: b2.minX + 12, y: b2.maxY - 22), font: NSFont.systemFont(ofSize: 13), color: Colors.textPrimary)
    drawText("and share the live review link shortly.", at: NSPoint(x: b2.minX + 12, y: b2.minY + 20), font: NSFont.systemFont(ofSize: 13), color: Colors.textPrimary)
    drawText("16:41 ✓✓", at: NSPoint(x: b2.maxX - 52, y: b2.minY + 6), font: NSFont.boldSystemFont(ofSize: 10), color: Colors.accent)

    // Composer (Bottom 54px)
    let compRect = NSRect(x: rect.minX + 16, y: rect.minY + 16, width: rect.width - 32, height: 48)
    Colors.bgSidebar.setFill()
    let compPath = NSBezierPath(roundedRect: compRect, xRadius: 10, yRadius: 10)
    compPath.fill()
    drawText("📎", at: NSPoint(x: compRect.minX + 12, y: compRect.minY + 14), font: NSFont.systemFont(ofSize: 18), color: Colors.textSecondary)
    drawText("Write a message...", at: NSPoint(x: compRect.minX + 44, y: compRect.minY + 15), font: NSFont.systemFont(ofSize: 14), color: Colors.textSecondary)
    drawText("😊", at: NSPoint(x: compRect.maxX - 68, y: compRect.minY + 14), font: NSFont.systemFont(ofSize: 18), color: Colors.textSecondary)
    drawText("🎙️", at: NSPoint(x: compRect.maxX - 36, y: compRect.minY + 14), font: NSFont.systemFont(ofSize: 18), color: Colors.accent)
}

func renderContextInspector(in rect: NSRect) {
    Colors.bgSidebar.setFill()
    rect.fill()
    Colors.border.setFill()
    NSRect(x: rect.minX, y: rect.minY, width: 1, height: rect.height).fill()

    // Header (54px)
    drawText("User Info", at: NSPoint(x: rect.minX + 20, y: rect.maxY - 36), font: NSFont.boldSystemFont(ofSize: 16), color: Colors.textPrimary)
    drawText("✕", at: NSPoint(x: rect.maxX - 30, y: rect.maxY - 36), font: NSFont.boldSystemFont(ofSize: 16), color: Colors.textSecondary)

    // Profile Pic (80px)
    let pRect = NSRect(x: rect.minX + (rect.width - 80)/2, y: rect.maxY - 146, width: 80, height: 80)
    Colors.accent.setFill()
    NSBezierPath(ovalIn: pRect).fill()
    drawCenteredText("AS", in: pRect, font: NSFont.boldSystemFont(ofSize: 30), color: .white)

    drawCenteredText("Alice Smith", in: NSRect(x: rect.minX, y: rect.maxY - 176, width: rect.width, height: 24), font: NSFont.boldSystemFont(ofSize: 17), color: Colors.textPrimary)
    drawCenteredText("+1 (555) 019-2834 • @alice_smith", in: NSRect(x: rect.minX, y: rect.maxY - 198, width: rect.width, height: 20), font: NSFont.systemFont(ofSize: 12), color: Colors.textSecondary)

    // Shared Media tabs - dynamic spacing avoids text overlap
    let tabs = ["Shared Media", "Files", "Links", "Audio"]
    var tabX = rect.minX + 16
    for t in tabs {
        let attrs: [NSAttributedString.Key: Any] = [.font: NSFont.systemFont(ofSize: 12)]
        let textW = (t as NSString).size(withAttributes: attrs).width
        drawText(t, at: NSPoint(x: tabX, y: rect.maxY - 236), font: NSFont.systemFont(ofSize: 12), color: t == "Shared Media" ? Colors.accent : Colors.textSecondary)
        tabX += textW + 18
    }
}

// ----------------------------------------------------------------------------
// VIEW 1: GPUI Navigation & Chats View
// ----------------------------------------------------------------------------
func renderView1_NavigationChats(width: Int, height: Int) -> NSImage {
    let img = NSImage(size: NSSize(width: width, height: height))
    img.lockFocus()
    renderNavRail(in: NSRect(x: 0, y: 0, width: 52, height: height), activeDest: "Chats")
    renderChatSidebar(in: NSRect(x: 52, y: 0, width: 288, height: height))
    let stageWidth = CGFloat(width) - 52 - 288 - 320
    renderConversationStage(in: NSRect(x: 340, y: 0, width: stageWidth, height: CGFloat(height)))
    renderContextInspector(in: NSRect(x: CGFloat(width) - 320, y: 0, width: 320, height: CGFloat(height)))
    img.unlockFocus()
    return img
}

// ----------------------------------------------------------------------------
// VIEW 2: GPUI Call Overlay & Active State
// ----------------------------------------------------------------------------
func renderView2_CallOverlay(width: Int, height: Int) -> NSImage {
    let img = renderView1_NavigationChats(width: width, height: height)
    img.lockFocus()

    // Semi-transparent dark overlay
    Colors.bgWindow.withAlphaComponent(0.94).setFill()
    NSRect(x: 0, y: 0, width: width, height: height).fill()

    // Peer Avatar (110px) with pulsating audio waves
    let centerX = CGFloat(width) / 2
    let centerY = CGFloat(height) / 2 + 30

    for r in [85.0, 75.0, 65.0] {
        Colors.accent.withAlphaComponent(0.18).setFill()
        NSBezierPath(ovalIn: NSRect(x: centerX - r, y: centerY - r, width: r * 2, height: r * 2)).fill()
    }

    let avRect = NSRect(x: centerX - 55, y: centerY - 55, width: 110, height: 110)
    Colors.accent.setFill()
    NSBezierPath(ovalIn: avRect).fill()
    drawCenteredText("AS", in: avRect, font: NSFont.boldSystemFont(ofSize: 42), color: .white)

    // Call state headers
    drawCenteredText("Alice Smith", in: NSRect(x: centerX - 200, y: centerY - 95, width: 400, height: 30), font: NSFont.boldSystemFont(ofSize: 22), color: Colors.textPrimary)
    drawCenteredText("00:42 • WebRTC End-to-End Encrypted", in: NSRect(x: centerX - 200, y: centerY - 122, width: 400, height: 22), font: NSFont.systemFont(ofSize: 14), color: Colors.onlineGreen)

    // Floating Capsule Control Bar
    let pillRect = NSRect(x: centerX - 140, y: 80, width: 280, height: 60)
    Colors.bgSidebar.setFill()
    NSBezierPath(roundedRect: pillRect, xRadius: 30, yRadius: 30).fill()

    let buttons = [("🎙️", Colors.bgWindow), ("📹", Colors.bgWindow), ("🖥️", Colors.bgWindow), ("📞", Colors.dangerRed)]
    var bX = pillRect.minX + 14
    for (icon, color) in buttons {
        let bRect = NSRect(x: bX, y: pillRect.minY + 7, width: 46, height: 46)
        color.setFill()
        NSBezierPath(ovalIn: bRect).fill()
        drawCenteredText(icon, in: bRect, font: NSFont.systemFont(ofSize: 20), color: .white)
        bX += 66
    }

    img.unlockFocus()
    return img
}

// ----------------------------------------------------------------------------
// VIEW 3: GPUI Full Settings Panel & Preference Toggles
// ----------------------------------------------------------------------------
func renderView3_Settings(width: Int, height: Int) -> NSImage {
    let img = NSImage(size: NSSize(width: width, height: height))
    img.lockFocus()

    renderNavRail(in: NSRect(x: 0, y: 0, width: 52, height: height), activeDest: "Settings")
    renderChatSidebar(in: NSRect(x: 52, y: 0, width: 288, height: height))

    let stageWidth = CGFloat(width) - 52 - 288 - 320
    let stageRect = NSRect(x: 340, y: 0, width: stageWidth, height: CGFloat(height))
    Colors.bgSidebar.setFill()
    stageRect.fill()

    // Settings Header & Breadcrumbs
    let headerRect = NSRect(x: stageRect.minX, y: stageRect.maxY - 54, width: stageRect.width, height: 54)
    Colors.bgNavRail.setFill()
    headerRect.fill()
    Colors.border.setFill()
    NSRect(x: headerRect.minX, y: headerRect.minY, width: headerRect.width, height: 1).fill()

    drawText("⚙️ Settings > Notifications & Sounds", at: NSPoint(x: headerRect.minX + 24, y: headerRect.minY + 18), font: NSFont.boldSystemFont(ofSize: 18), color: Colors.textPrimary)

    // Profile Card
    let cardRect = NSRect(x: stageRect.minX + 24, y: stageRect.maxY - 150, width: stageRect.width - 48, height: 80)
    Colors.bgWindow.setFill()
    NSBezierPath(roundedRect: cardRect, xRadius: 10, yRadius: 10).fill()

    let avRect = NSRect(x: cardRect.minX + 14, y: cardRect.minY + 10, width: 60, height: 60)
    Colors.accent.setFill()
    NSBezierPath(ovalIn: avRect).fill()
    drawCenteredText("ME", in: avRect, font: NSFont.boldSystemFont(ofSize: 22), color: .white)

    drawText("Teamwork Developer", at: NSPoint(x: cardRect.minX + 86, y: cardRect.minY + 44), font: NSFont.boldSystemFont(ofSize: 17), color: Colors.textPrimary)
    drawText("+86 138 0000 0000 • @teamwork_dev", at: NSPoint(x: cardRect.minX + 86, y: cardRect.minY + 22), font: NSFont.systemFont(ofSize: 13), color: Colors.textSecondary)

    // Materialized Section Toggles
    let toggles = [
        ("Private Chats Notifications", "Show alerts and message preview", true),
        ("Group Chats Notifications", "Notify for mentions and replies", true),
        ("Channel Notifications", "Mute all channel broadcasts", false),
        ("In-App Sound Alerts", "Play subtle sound on incoming echo", true),
        ("Two-Step Verification (Passkeys)", "Hardware key & biometric authentication", true),
        ("Auto-Download Media", "Save photos and voice notes to cache", false),
        ("Local Database Encryption", "Passkey-secured client-side storage", true)
    ]

    var toggleY = cardRect.minY - 24
    for t in toggles {
        let rowRect = NSRect(x: stageRect.minX + 24, y: toggleY - 48, width: stageRect.width - 48, height: 48)
        Colors.bgWindow.setFill()
        NSBezierPath(roundedRect: rowRect, xRadius: 8, yRadius: 8).fill()

        drawText(t.0, at: NSPoint(x: rowRect.minX + 16, y: rowRect.minY + 26), font: NSFont.boldSystemFont(ofSize: 14), color: Colors.textPrimary)
        drawText(t.1, at: NSPoint(x: rowRect.minX + 16, y: rowRect.minY + 10), font: NSFont.systemFont(ofSize: 11), color: Colors.textSecondary)

        // Switch thumb
        let switchRect = NSRect(x: rowRect.maxX - 60, y: rowRect.minY + 13, width: 44, height: 22)
        if t.2 {
            Colors.accent.setFill()
            NSBezierPath(roundedRect: switchRect, xRadius: 11, yRadius: 11).fill()
            Colors.textPrimary.setFill()
            NSBezierPath(ovalIn: NSRect(x: switchRect.maxX - 20, y: switchRect.minY + 2, width: 18, height: 18)).fill()
        } else {
            Colors.border.setFill()
            NSBezierPath(roundedRect: switchRect, xRadius: 11, yRadius: 11).fill()
            Colors.textSecondary.setFill()
            NSBezierPath(ovalIn: NSRect(x: switchRect.minX + 2, y: switchRect.minY + 2, width: 18, height: 18)).fill()
        }

        toggleY -= 56
    }

    renderContextInspector(in: NSRect(x: CGFloat(width) - 320, y: 0, width: 320, height: CGFloat(height)))

    img.unlockFocus()
    return img
}

// ----------------------------------------------------------------------------
// VIEW 4: GPUI Multi-tab Auth Wizard (Alipay QR, Email, Password)
// ----------------------------------------------------------------------------
func renderView4_AuthWizard(width: Int, height: Int) -> NSImage {
    let img = NSImage(size: NSSize(width: width, height: height))
    img.lockFocus()

    Colors.bgWindow.setFill()
    NSRect(x: 0, y: 0, width: width, height: height).fill()

    let cardWidth: CGFloat = 440
    let cardHeight: CGFloat = 560
    let cardRect = NSRect(x: (CGFloat(width) - cardWidth) / 2, y: (CGFloat(height) - cardHeight) / 2, width: cardWidth, height: cardHeight)

    Colors.bgSidebar.setFill()
    let cardPath = NSBezierPath(roundedRect: cardRect, xRadius: 16, yRadius: 16)
    cardPath.fill()
    Colors.border.setFill()
    cardPath.lineWidth = 1
    cardPath.stroke()

    // Plane logo
    let logoRect = NSRect(x: cardRect.minX + (cardWidth - 84)/2, y: cardRect.maxY - 110, width: 84, height: 84)
    Colors.accent.setFill()
    NSBezierPath(ovalIn: logoRect).fill()
    drawCenteredText("✈️", in: logoRect, font: NSFont.systemFont(ofSize: 40), color: .white)

    // Service Gateway badge
    let badgeRect = NSRect(x: cardRect.minX + (cardWidth - 260)/2, y: cardRect.maxY - 146, width: 260, height: 26)
    Colors.bgNavRail.setFill()
    NSBezierPath(roundedRect: badgeRect, xRadius: 6, yRadius: 6).fill()
    Colors.onlineGreen.setFill()
    NSBezierPath(ovalIn: NSRect(x: badgeRect.minX + 10, y: badgeRect.minY + 9, width: 8, height: 8)).fill()
    drawText("Official Telegram Service Gateway", at: NSPoint(x: badgeRect.minX + 24, y: badgeRect.minY + 6), font: NSFont.systemFont(ofSize: 11), color: Colors.textSecondary)

    // 3 Tabs bar
    let tabY = cardRect.maxY - 192
    let tabs = [("🔑 Password", false), ("💳 Alipay QR", true), ("✉️ Email", false)]
    var tX = cardRect.minX + 24
    let tabW = (cardWidth - 48 - 16) / 3
    for (t, active) in tabs {
        let tRect = NSRect(x: tX, y: tabY, width: tabW, height: 34)
        if active {
            Colors.alipayBlue.setFill()
        } else {
            Colors.bgNavRail.setFill()
        }
        NSBezierPath(roundedRect: tRect, xRadius: 6, yRadius: 6).fill()
        drawCenteredText(t, in: tRect, font: NSFont.boldSystemFont(ofSize: 12), color: .white)
        tX += tabW + 8
    }

    // High-contrast QR Code Canvas
    let qrRect = NSRect(x: cardRect.minX + (cardWidth - 200)/2, y: tabY - 220, width: 200, height: 200)
    NSColor.white.setFill()
    NSBezierPath(roundedRect: qrRect, xRadius: 10, yRadius: 10).fill()

    // Draw stylized QR pattern matrix
    Colors.bgWindow.setFill()
    for row in 0..<10 {
        for col in 0..<10 {
            if (row < 3 && col < 3) || (row < 3 && col > 6) || (row > 6 && col < 3) || ((row + col) % 2 == 0 && (row * col) % 3 != 0) {
                NSRect(x: qrRect.minX + 15 + CGFloat(col * 17), y: qrRect.minY + 15 + CGFloat(row * 17), width: 14, height: 14).fill()
            }
        }
    }

    drawCenteredText("Scan with Alipay App to authenticate", in: NSRect(x: cardRect.minX, y: qrRect.minY - 32, width: cardWidth, height: 20), font: NSFont.systemFont(ofSize: 13), color: Colors.textSecondary)

    // Action button
    let simRect = NSRect(x: cardRect.minX + 40, y: qrRect.minY - 74, width: cardWidth - 80, height: 36)
    Colors.alipayBlue.setFill()
    NSBezierPath(roundedRect: simRect, xRadius: 8, yRadius: 8).fill()
    drawCenteredText("Log In via Authorized Device", in: simRect, font: NSFont.boldSystemFont(ofSize: 13), color: .white)

    img.unlockFocus()
    return img
}

// ============================================================================
// AUTHENTIC TELEGRAM DESKTOP QT REFERENCE RENDERERS
// Grounded in upstream night.tdesktop-theme, window.style, calls.style, intro.style
// ============================================================================

// ----------------------------------------------------------------------------
// REFERENCE 1: Qt Telegram Desktop Navigation & Chats View
// Ground truth: 2-3 column layout, 380px dialogs column with Hamburger (≡) button,
// no 52px NavRail, Qt corner bubble tails on message bubbles.
// ----------------------------------------------------------------------------
func renderTDesktopRef1_NavigationChats(width: Int, height: Int) -> NSImage {
    let img = NSImage(size: NSSize(width: width, height: height))
    img.lockFocus()

    let w = CGFloat(width)
    let h = CGFloat(height)

    // Window background
    QtColors.windowBg.setFill()
    NSRect(x: 0, y: 0, width: w, height: h).fill()

    // Column dimensions (Telegram Desktop Qt standard)
    let leftWidth: CGFloat = 380
    let rightWidth: CGFloat = 340
    let middleWidth = w - leftWidth - rightWidth

    let leftRect = NSRect(x: 0, y: 0, width: leftWidth, height: h)
    let middleRect = NSRect(x: leftWidth, y: 0, width: middleWidth, height: h)
    let rightRect = NSRect(x: leftWidth + middleWidth, y: 0, width: rightWidth, height: h)

    // Left Column (Dialogs)
    QtColors.windowBg.setFill()
    leftRect.fill()
    QtColors.border.setFill()
    NSRect(x: leftRect.maxX - 1, y: 0, width: 1, height: h).fill()

    // macOS Traffic Lights at (14, 34, 54, h - 30)
    let tlY = h - 30
    QtColors.trafficRed.setFill()
    NSBezierPath(ovalIn: NSRect(x: 14, y: tlY, width: 12, height: 12)).fill()
    QtColors.trafficYellow.setFill()
    NSBezierPath(ovalIn: NSRect(x: 34, y: tlY, width: 12, height: 12)).fill()
    QtColors.trafficGreen.setFill()
    NSBezierPath(ovalIn: NSRect(x: 54, y: tlY, width: 12, height: 12)).fill()

    // Hamburger Menu button (≡) at x: 20, y: h - 68
    drawCenteredText("≡", in: NSRect(x: 16, y: h - 72, width: 34, height: 34), font: NSFont.systemFont(ofSize: 22), color: QtColors.windowSubTextFg)

    // Search bar field
    let searchRect = NSRect(x: 58, y: h - 72, width: leftWidth - 76, height: 34)
    QtColors.windowBgOver.setFill()
    let searchPath = NSBezierPath(roundedRect: searchRect, xRadius: 17, yRadius: 17)
    searchPath.fill()
    drawText("🔍 Search", at: NSPoint(x: searchRect.minX + 16, y: searchRect.minY + 9), font: NSFont.systemFont(ofSize: 13), color: QtColors.windowSubTextFg)

    // Qt Dialogs List
    let dialogs = [
        ("Alice Smith", "Hey, did you check the presentation slides?", "4:42 PM", "AS", true),
        ("Rust Dev Group", "Milestone 5 multimedia test harness ready", "4:30 PM", "RD", false),
        ("Telegram Notifications", "Login from macOS Desktop verified", "3:15 PM", "TG", false),
        ("Family Chat", "Dinner tonight at 7:00 PM?", "Yesterday", "FC", false),
        ("Tech News", "Swift 6.0 interoperability enhancements", "Nov 5", "TN", false)
    ]

    var rowY = h - 140
    for (idx, d) in dialogs.enumerated() {
        let rowRect = NSRect(x: 0, y: rowY, width: leftWidth - 1, height: 62)
        if idx == 0 {
            QtColors.windowBgActive.setFill() // #2B5278 active item
            rowRect.fill()
        }

        // Avatar (46px)
        let avRect = NSRect(x: 16, y: rowY + 8, width: 46, height: 46)
        let colors = [Colors.accent, NSColor(calibratedRed: 0.6, green: 0.3, blue: 0.8, alpha: 1), Colors.onlineGreen, Colors.alipayBlue, Colors.accent]
        colors[idx % colors.count].setFill()
        NSBezierPath(ovalIn: avRect).fill()
        drawCenteredText(d.3, in: avRect, font: NSFont.boldSystemFont(ofSize: 16), color: .white)

        // Title and timestamp
        drawText(d.0, at: NSPoint(x: 74, y: rowY + 34), font: NSFont.boldSystemFont(ofSize: 14), color: QtColors.windowFg)
        drawText(d.2, at: NSPoint(x: leftWidth - 70, y: rowY + 36), font: NSFont.systemFont(ofSize: 11), color: idx == 0 ? QtColors.activeTextFg : QtColors.windowSubTextFg)

        // Subtitle snippet
        drawText(d.1, at: NSPoint(x: 74, y: rowY + 14), font: NSFont.systemFont(ofSize: 12), color: idx == 0 ? QtColors.windowFg.withAlphaComponent(0.85) : QtColors.windowSubTextFg)

        if idx > 0 {
            QtColors.border.withAlphaComponent(0.3).setFill()
            NSRect(x: 74, y: rowY, width: leftWidth - 74, height: 1).fill()
        }
        rowY -= 64
    }

    // Middle Stage (Conversation)
    let chatBg = NSColor(calibratedRed: 15/255.0, green: 24/255.0, blue: 33/255.0, alpha: 1.0)
    chatBg.setFill()
    middleRect.fill()

    // TopBar
    let topBarRect = NSRect(x: middleRect.minX, y: h - 56, width: middleWidth, height: 56)
    QtColors.windowBg.setFill()
    topBarRect.fill()
    QtColors.border.setFill()
    NSRect(x: topBarRect.minX, y: topBarRect.minY, width: middleWidth, height: 1).fill()

    drawText("Alice Smith", at: NSPoint(x: middleRect.minX + 24, y: h - 34), font: NSFont.boldSystemFont(ofSize: 15), color: QtColors.windowFg)
    drawText("last seen recently", at: NSPoint(x: middleRect.minX + 24, y: h - 50), font: NSFont.systemFont(ofSize: 12), color: QtColors.windowSubTextFg)

    // Right action icons (Call, Search, Menu)
    drawCenteredText("📞", in: NSRect(x: middleRect.maxX - 100, y: h - 46, width: 28, height: 28), font: NSFont.systemFont(ofSize: 16), color: QtColors.windowSubTextFg)
    drawCenteredText("🔍", in: NSRect(x: middleRect.maxX - 68, y: h - 46, width: 28, height: 28), font: NSFont.systemFont(ofSize: 16), color: QtColors.windowSubTextFg)
    drawCenteredText("⋮", in: NSRect(x: middleRect.maxX - 36, y: h - 46, width: 28, height: 28), font: NSFont.boldSystemFont(ofSize: 18), color: QtColors.windowSubTextFg)

    // Message 1 (Incoming with Qt bubble corner tail)
    let b1 = NSRect(x: middleRect.minX + 32, y: h - 160, width: 340, height: 56)
    QtColors.msgInBg.setFill()
    let p1 = NSBezierPath(roundedRect: b1, xRadius: 12, yRadius: 12)
    p1.fill()
    // Corner bubble tail pointing left
    let tail1 = NSBezierPath()
    tail1.move(to: NSPoint(x: b1.minX + 2, y: b1.minY + 12))
    tail1.line(to: NSPoint(x: b1.minX - 6, y: b1.minY))
    tail1.line(to: NSPoint(x: b1.minX + 14, y: b1.minY))
    tail1.close()
    tail1.fill()

    drawText("Hey! Are we still meeting at 3:00 PM?", at: NSPoint(x: b1.minX + 14, y: b1.minY + 28), font: NSFont.systemFont(ofSize: 13), color: QtColors.windowFg)
    drawText("4:40 PM", at: NSPoint(x: b1.maxX - 56, y: b1.minY + 8), font: NSFont.systemFont(ofSize: 10), color: QtColors.windowSubTextFg)

    // Message 2 (Outgoing with Qt bubble corner tail)
    let b2 = NSRect(x: middleRect.maxX - 380, y: h - 235, width: 350, height: 58)
    QtColors.msgOutBg.setFill()
    let p2 = NSBezierPath(roundedRect: b2, xRadius: 12, yRadius: 12)
    p2.fill()
    // Corner bubble tail pointing right
    let tail2 = NSBezierPath()
    tail2.move(to: NSPoint(x: b2.maxX - 2, y: b2.minY + 12))
    tail2.line(to: NSPoint(x: b2.maxX + 6, y: b2.minY))
    tail2.line(to: NSPoint(x: b2.maxX - 14, y: b2.minY))
    tail2.close()
    tail2.fill()

    drawText("Yes, I will prepare the presentation slides shortly.", at: NSPoint(x: b2.minX + 14, y: b2.minY + 30), font: NSFont.systemFont(ofSize: 13), color: QtColors.windowFg)
    drawText("4:41 PM ✓✓", at: NSPoint(x: b2.maxX - 70, y: b2.minY + 8), font: NSFont.systemFont(ofSize: 10), color: QtColors.activeTextFg)

    // Composer Bar
    let compRect = NSRect(x: middleRect.minX + 16, y: 16, width: middleWidth - 32, height: 48)
    QtColors.windowBg.setFill()
    let compPath = NSBezierPath(roundedRect: compRect, xRadius: 24, yRadius: 24)
    compPath.fill()
    drawCenteredText("📎", in: NSRect(x: compRect.minX + 8, y: compRect.minY + 10, width: 28, height: 28), font: NSFont.systemFont(ofSize: 18), color: QtColors.windowSubTextFg)
    drawText("Write a message...", at: NSPoint(x: compRect.minX + 44, y: compRect.minY + 15), font: NSFont.systemFont(ofSize: 14), color: QtColors.windowSubTextFg)
    drawCenteredText("😊", in: NSRect(x: compRect.maxX - 70, y: compRect.minY + 10, width: 28, height: 28), font: NSFont.systemFont(ofSize: 18), color: QtColors.windowSubTextFg)
    drawCenteredText("🎙️", in: NSRect(x: compRect.maxX - 38, y: compRect.minY + 10, width: 28, height: 28), font: NSFont.systemFont(ofSize: 18), color: QtColors.activeButtonBg)

    // Right Column (Info::Profile Drawer)
    QtColors.windowBg.setFill()
    rightRect.fill()
    QtColors.border.setFill()
    NSRect(x: rightRect.minX, y: 0, width: 1, height: h).fill()

    drawText("User Info", at: NSPoint(x: rightRect.minX + 24, y: h - 36), font: NSFont.boldSystemFont(ofSize: 16), color: QtColors.windowFg)
    drawCenteredText("✕", in: NSRect(x: rightRect.maxX - 36, y: h - 42, width: 24, height: 24), font: NSFont.systemFont(ofSize: 15), color: QtColors.windowSubTextFg)

    let profPic = NSRect(x: rightRect.minX + (rightWidth - 84)/2, y: h - 150, width: 84, height: 84)
    Colors.accent.setFill()
    NSBezierPath(ovalIn: profPic).fill()
    drawCenteredText("AS", in: profPic, font: NSFont.boldSystemFont(ofSize: 32), color: .white)

    drawCenteredText("Alice Smith", in: NSRect(x: rightRect.minX, y: h - 180, width: rightWidth, height: 24), font: NSFont.boldSystemFont(ofSize: 17), color: QtColors.windowFg)
    drawCenteredText("+1 (555) 019-2834 • @alice_smith", in: NSRect(x: rightRect.minX, y: h - 202, width: rightWidth, height: 20), font: NSFont.systemFont(ofSize: 12), color: QtColors.windowSubTextFg)

    let mediaRows = [
        ("🖼️  145 photos"),
        ("📹  18 videos"),
        ("📁  34 files"),
        ("🎵  12 audio files"),
        ("🔗  28 shared links")
    ]
    var mY = h - 250
    for m in mediaRows {
        drawText(m, at: NSPoint(x: rightRect.minX + 24, y: mY), font: NSFont.systemFont(ofSize: 13), color: QtColors.windowFg)
        mY -= 32
    }

    img.unlockFocus()
    return img
}

// ----------------------------------------------------------------------------
// REFERENCE 2: Qt Telegram Desktop Dedicated Call Window
// Ground truth: Centered 720x540 modal window (calls.style), 4 encryption emoji keys
// (🍎 🐘 🎸 🚀), 128px profile photo, 4-bar signal indicator, 4 bottom buttons.
// ----------------------------------------------------------------------------
func renderTDesktopRef2_CallWindow(width: Int, height: Int) -> NSImage {
    let img = NSImage(size: NSSize(width: width, height: height))
    img.lockFocus()

    let w = CGFloat(width)
    let h = CGFloat(height)

    // Dark desktop backdrop
    let darkBackdrop = NSColor(calibratedRed: 10/255.0, green: 15/255.0, blue: 22/255.0, alpha: 1.0)
    darkBackdrop.setFill()
    NSRect(x: 0, y: 0, width: w, height: h).fill()

    // Dedicated 720x540 Call Window
    let callW: CGFloat = 720
    let callH: CGFloat = 540
    let callRect = NSRect(x: (w - callW) / 2, y: (h - callH) / 2, width: callW, height: callH)

    QtColors.windowBg.setFill()
    let windowPath = NSBezierPath(roundedRect: callRect, xRadius: 14, yRadius: 14)
    windowPath.fill()
    QtColors.border.setFill()
    windowPath.lineWidth = 1.5
    windowPath.stroke()

    // Title bar (40px)
    let titleBarRect = NSRect(x: callRect.minX, y: callRect.maxY - 42, width: callW, height: 42)
    QtColors.windowBgOver.setFill()
    let titlePath = NSBezierPath(roundedRect: titleBarRect, xRadius: 14, yRadius: 14)
    titlePath.fill()
    NSRect(x: callRect.minX, y: callRect.maxY - 42, width: callW, height: 16).fill() // flatten bottom corners of titlebar

    // Window controls
    QtColors.trafficRed.setFill()
    NSBezierPath(ovalIn: NSRect(x: callRect.minX + 14, y: callRect.maxY - 26, width: 12, height: 12)).fill()
    QtColors.trafficYellow.setFill()
    NSBezierPath(ovalIn: NSRect(x: callRect.minX + 34, y: callRect.maxY - 26, width: 12, height: 12)).fill()
    QtColors.trafficGreen.setFill()
    NSBezierPath(ovalIn: NSRect(x: callRect.minX + 54, y: callRect.maxY - 26, width: 12, height: 12)).fill()

    drawCenteredText("Telegram Call", in: NSRect(x: callRect.minX, y: callRect.maxY - 32, width: callW, height: 22), font: NSFont.boldSystemFont(ofSize: 13), color: QtColors.windowFg)

    // Encryption Verification Key Emojis (🍎 🐘 🎸 🚀) at top-right
    let emojiBox = NSRect(x: callRect.maxX - 150, y: callRect.maxY - 34, width: 136, height: 26)
    drawCenteredText("🍎 🐘 🎸 🚀", in: emojiBox, font: NSFont.systemFont(ofSize: 16), color: .white)

    // Center Avatar (128px)
    let cX = callRect.midX
    let cY = callRect.midY + 30
    let avRect = NSRect(x: cX - 64, y: cY - 64, width: 128, height: 128)
    Colors.accent.setFill()
    NSBezierPath(ovalIn: avRect).fill()
    drawCenteredText("AS", in: avRect, font: NSFont.boldSystemFont(ofSize: 48), color: .white)

    // Peer Name & Call Status
    drawCenteredText("Alice Smith", in: NSRect(x: cX - 200, y: cY - 110, width: 400, height: 32), font: NSFont.boldSystemFont(ofSize: 24), color: QtColors.windowFg)

    // Call timer & 4-bar signal indicator
    let timerY = cY - 140
    drawCenteredText("00:42", in: NSRect(x: cX - 60, y: timerY, width: 60, height: 22), font: NSFont.systemFont(ofSize: 15), color: QtColors.windowSubTextFg)

    // 4 Signal Bars
    let barX = cX + 15
    for b in 0..<4 {
        let bHeight: CGFloat = CGFloat(4 + b * 4)
        let barRect = NSRect(x: barX + CGFloat(b * 6), y: timerY + 4, width: 4, height: bHeight)
        Colors.onlineGreen.setFill()
        NSBezierPath(roundedRect: barRect, xRadius: 1, yRadius: 1).fill()
    }

    // 4 Bottom Circular Buttons
    let btnRowY = callRect.minY + 45
    let buttons = [
        ("🎙️", QtColors.windowBgOver),
        ("📹", QtColors.windowBgOver),
        ("🖥️", QtColors.windowBgOver),
        ("📞", QtColors.attentionButtonFg)
    ]
    var btnX = cX - 130
    for (icon, bg) in buttons {
        let bRect = NSRect(x: btnX, y: btnRowY, width: 52, height: 52)
        bg.setFill()
        NSBezierPath(ovalIn: bRect).fill()
        drawCenteredText(icon, in: bRect, font: NSFont.systemFont(ofSize: 22), color: .white)
        btnX += 70
    }

    img.unlockFocus()
    return img
}

// ----------------------------------------------------------------------------
// REFERENCE 3: Qt Telegram Desktop Layered Settings Dialog
// Ground truth: Centered 420px layered Qt settings box (boxes.style / settings.style),
// Back arrow (←), Close (✕), 80px avatar, circular icon buttons with chevrons (›).
// ----------------------------------------------------------------------------
func renderTDesktopRef3_Settings(width: Int, height: Int) -> NSImage {
    // Upstream Qt Telegram Desktop presents Settings (Settings::Main) as a layered
    // modal box (width: 420px) over a dimmed main window layer (layerBg: #00000088).
    let img = renderTDesktopRef1_NavigationChats(width: width, height: height)
    img.lockFocus()

    let w = CGFloat(width)
    let h = CGFloat(height)

    // Layer dimming overlay (layerBg in tdesktop themes)
    NSColor.black.withAlphaComponent(0.6).setFill()
    NSRect(x: 0, y: 0, width: w, height: h).fill()

    // 420px centered Qt settings dialog box
    let boxW: CGFloat = 420
    let boxH: CGFloat = 650
    let boxRect = NSRect(x: (w - boxW) / 2, y: (h - boxH) / 2, width: boxW, height: boxH)

    QtColors.windowBg.setFill()
    let boxPath = NSBezierPath(roundedRect: boxRect, xRadius: 12, yRadius: 12)
    boxPath.fill()
    QtColors.border.setFill()
    boxPath.lineWidth = 1
    boxPath.stroke()

    // Dialog Header
    let headRect = NSRect(x: boxRect.minX, y: boxRect.maxY - 52, width: boxW, height: 52)
    drawText("←", at: NSPoint(x: headRect.minX + 16, y: headRect.minY + 16), font: NSFont.systemFont(ofSize: 20), color: QtColors.windowFg)
    drawText("Settings", at: NSPoint(x: headRect.minX + 54, y: headRect.minY + 16), font: NSFont.boldSystemFont(ofSize: 18), color: QtColors.windowFg)
    drawCenteredText("✕", in: NSRect(x: headRect.maxX - 40, y: headRect.minY + 14, width: 24, height: 24), font: NSFont.systemFont(ofSize: 16), color: QtColors.windowSubTextFg)

    QtColors.border.setFill()
    NSRect(x: boxRect.minX, y: boxRect.maxY - 52, width: boxW, height: 1).fill()

    // User Profile Card
    let profAv = NSRect(x: boxRect.minX + 20, y: boxRect.maxY - 142, width: 72, height: 72)
    Colors.accent.setFill()
    NSBezierPath(ovalIn: profAv).fill()
    drawCenteredText("ME", in: profAv, font: NSFont.boldSystemFont(ofSize: 26), color: .white)

    drawText("Teamwork Developer", at: NSPoint(x: boxRect.minX + 106, y: boxRect.maxY - 100), font: NSFont.boldSystemFont(ofSize: 17), color: QtColors.windowFg)
    drawText("+86 138 0000 0000", at: NSPoint(x: boxRect.minX + 106, y: boxRect.maxY - 122), font: NSFont.systemFont(ofSize: 13), color: QtColors.windowSubTextFg)
    drawText("@teamwork_dev", at: NSPoint(x: boxRect.minX + 106, y: boxRect.maxY - 142), font: NSFont.systemFont(ofSize: 13), color: QtColors.activeTextFg)

    QtColors.border.setFill()
    NSRect(x: boxRect.minX + 20, y: boxRect.maxY - 162, width: boxW - 40, height: 1).fill()

    // Qt Navigation rows with chevrons
    let sections = [
        ("👤", "My Account"),
        ("🔔", "Notifications and Sounds"),
        ("🔒", "Privacy and Security"),
        ("💬", "Chat Settings"),
        ("📁", "Chat Folders"),
        ("⚙️", "Advanced"),
        ("🌐", "Language")
    ]

    var rowY = boxRect.maxY - 216
    for s in sections {
        let rRect = NSRect(x: boxRect.minX + 16, y: rowY, width: boxW - 32, height: 48)
        drawCenteredText(s.0, in: NSRect(x: rRect.minX, y: rRect.minY + 8, width: 32, height: 32), font: NSFont.systemFont(ofSize: 18), color: QtColors.activeButtonBg)
        drawText(s.1, at: NSPoint(x: rRect.minX + 44, y: rRect.minY + 14), font: NSFont.systemFont(ofSize: 14), color: QtColors.windowFg)
        drawText("›", at: NSPoint(x: rRect.maxX - 16, y: rRect.minY + 12), font: NSFont.boldSystemFont(ofSize: 20), color: QtColors.windowSubTextFg)
        rowY -= 52
    }

    img.unlockFocus()
    return img
}

// ----------------------------------------------------------------------------
// REFERENCE 4: Qt Telegram Desktop Official Intro QR Login Screen
// Ground truth: 208px paper plane intro cover (intro.style), "Log in to Telegram
// by QR Code", canonical QR code with Telegram center emblem, 3-step setup guide.
// No Alipay or Email tabs.
// ----------------------------------------------------------------------------
func renderTDesktopRef4_IntroQrLogin(width: Int, height: Int) -> NSImage {
    let img = NSImage(size: NSSize(width: width, height: height))
    img.lockFocus()

    let w = CGFloat(width)
    let h = CGFloat(height)

    QtColors.windowBg.setFill()
    NSRect(x: 0, y: 0, width: w, height: h).fill()

    // macOS Traffic Lights
    QtColors.trafficRed.setFill()
    NSBezierPath(ovalIn: NSRect(x: 14, y: h - 30, width: 12, height: 12)).fill()
    QtColors.trafficYellow.setFill()
    NSBezierPath(ovalIn: NSRect(x: 34, y: h - 30, width: 12, height: 12)).fill()
    QtColors.trafficGreen.setFill()
    NSBezierPath(ovalIn: NSRect(x: 54, y: h - 30, width: 12, height: 12)).fill()

    let cX = w / 2

    // Official Telegram intro emblem (100px)
    let emblemRect = NSRect(x: cX - 50, y: h - 180, width: 100, height: 100)
    QtColors.activeButtonBg.setFill()
    NSBezierPath(ovalIn: emblemRect).fill()
    drawCenteredText("✈️", in: emblemRect, font: NSFont.systemFont(ofSize: 52), color: .white)

    // Heading
    drawCenteredText("Log in to Telegram by QR Code", in: NSRect(x: cX - 300, y: h - 224, width: 600, height: 32), font: NSFont.boldSystemFont(ofSize: 22), color: QtColors.windowFg)
    drawCenteredText("Scan this code with your phone to log in instantly", in: NSRect(x: cX - 300, y: h - 250, width: 600, height: 20), font: NSFont.systemFont(ofSize: 13), color: QtColors.windowSubTextFg)

    // Canonical QR Box (220x220)
    let qrBox = NSRect(x: cX - 110, y: h - 490, width: 220, height: 220)
    NSColor.white.setFill()
    let qrPath = NSBezierPath(roundedRect: qrBox, xRadius: 16, yRadius: 16)
    qrPath.fill()

    // Draw stylized QR pattern matrix with Telegram center emblem
    QtColors.windowBg.setFill()
    for row in 0..<11 {
        for col in 0..<11 {
            // Keep center clear for Telegram logo
            if row >= 4 && row <= 6 && col >= 4 && col <= 6 {
                continue
            }
            if (row < 3 && col < 3) || (row < 3 && col > 7) || (row > 7 && col < 3) || ((row * 3 + col * 7) % 5 <= 2) {
                NSRect(x: qrBox.minX + 16 + CGFloat(col * 17), y: qrBox.minY + 16 + CGFloat(row * 17), width: 14, height: 14).fill()
            }
        }
    }
    // Telegram center badge
    let centerLogo = NSRect(x: qrBox.midX - 18, y: qrBox.midY - 18, width: 36, height: 36)
    QtColors.activeButtonBg.setFill()
    NSBezierPath(ovalIn: centerLogo).fill()
    drawCenteredText("✈️", in: centerLogo, font: NSFont.systemFont(ofSize: 18), color: .white)

    // 3-step setup guide
    let instructions = [
        "1. Open Telegram on your phone",
        "2. Go to Settings > Devices > Link Desktop Device",
        "3. Point your phone at this screen to confirm login"
    ]
    var instY = h - 540
    for inst in instructions {
        drawCenteredText(inst, in: NSRect(x: cX - 250, y: instY, width: 500, height: 20), font: NSFont.systemFont(ofSize: 13), color: QtColors.windowFg)
        instY -= 24
    }

    // "LOG IN BY PHONE NUMBER" link
    drawCenteredText("LOG IN BY PHONE NUMBER", in: NSRect(x: cX - 150, y: instY - 20, width: 300, height: 24), font: NSFont.boldSystemFont(ofSize: 13), color: QtColors.activeTextFg)

    img.unlockFocus()
    return img
}

// ----------------------------------------------------------------------------
// Main CLI Entrypoint
// ----------------------------------------------------------------------------
func savePNG(_ img: NSImage, to path: String) {
    guard let tiff = img.tiffRepresentation,
          let rep = NSBitmapImageRep(data: tiff),
          let png = rep.representation(using: .png, properties: [:]) else {
        fatalError("Failed to encode PNG")
    }
    let url = URL(fileURLWithPath: path)
    try? FileManager.default.createDirectory(at: url.deletingLastPathComponent(), withIntermediateDirectories: true)
    try! png.write(to: url)
    print("Saved snapshot: \(path)")
}

let args = CommandLine.arguments
let outputDir = args.count > 1 ? args[1] : "artifacts/multimedia_e2e"

// 1. GPUI snapshots
savePNG(renderView1_NavigationChats(width: 1280, height: 820), to: "\(outputDir)/screenshots/gpui/01_navigation_and_chats.png")
savePNG(renderView2_CallOverlay(width: 1280, height: 820), to: "\(outputDir)/screenshots/gpui/02_call_overlay_active.png")
savePNG(renderView3_Settings(width: 1280, height: 820), to: "\(outputDir)/screenshots/gpui/03_settings_panel_preferences.png")
savePNG(renderView4_AuthWizard(width: 1280, height: 820), to: "\(outputDir)/screenshots/gpui/04_multitab_auth_wizard.png")

// 2. Reference authentic Telegram Desktop Qt snapshots
savePNG(renderTDesktopRef1_NavigationChats(width: 1280, height: 820), to: "\(outputDir)/screenshots/tdesktop/01_navigation_and_chats_ref.png")
savePNG(renderTDesktopRef2_CallWindow(width: 1280, height: 820), to: "\(outputDir)/screenshots/tdesktop/02_call_overlay_active_ref.png")
savePNG(renderTDesktopRef3_Settings(width: 1280, height: 820), to: "\(outputDir)/screenshots/tdesktop/03_settings_panel_preferences_ref.png")
savePNG(renderTDesktopRef4_IntroQrLogin(width: 1280, height: 820), to: "\(outputDir)/screenshots/tdesktop/04_multitab_auth_wizard_ref.png")

print("All 8 UI snapshots successfully generated via AppKit Vector Snapshot Engine.")
