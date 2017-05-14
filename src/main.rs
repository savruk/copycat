extern crate cocoa;

use cocoa::base::{selector, nil, id, NO};
use cocoa::foundation::{NSAutoreleasePool, NSRect, NSPoint, NSSize, NSUInteger,
                        NSProcessInfo, NSString};

use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyAccessory,
                    NSMenu, NSMenuItem, NSWindow, NSBackingStoreBuffered,
                    NSStatusItem, NSStatusBar, NSTitledWindowMask,
                    NSVariableStatusItemLength, NSButton};
//use base::{NSString};

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyAccessory);
        add_status_item();
        add_window();
        app.run();
    }
}

unsafe fn add_window() {
    let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
        NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
        NSTitledWindowMask as NSUInteger,
        NSBackingStoreBuffered,
        NO
    ).autorelease();
    window.center();
    let title = NSString::alloc(nil).init_str("Hello World!");
    NSWindow::setTitle_(window, title);
    window.makeKeyAndOrderFront_(nil);
}

unsafe fn add_status_item() {
    let status_bar = NSStatusBar::systemStatusBar(nil);
    let status_item = status_bar.statusItemWithLength_(NSVariableStatusItemLength);
    let button = status_item.button();
    NSButton::setTitle_(button, NSString::alloc(nil).init_str("YOYO"));
    add_menu(&status_item)
}

/// Add a menu to an application instance
unsafe fn add_menu(status_item: &id) {
    // create Menu Bar
    let menubar = NSMenu::new(nil).autorelease();
    let app_menu_item = NSMenuItem::new(nil).autorelease();
    menubar.addItem_(app_menu_item);
    status_item.setMenu_(menubar);

    // create Application menu
    let app_menu = NSMenu::new(nil).autorelease();
    let quit_prefix = NSString::alloc(nil).init_str("Quit ");
    let quit_title = quit_prefix.stringByAppendingString_(
        NSProcessInfo::processInfo(nil).processName()
    );
    let quit_action = selector("terminate:");
    let quit_key = NSString::alloc(nil).init_str("q");
    let quit_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
        quit_title,
        quit_action,
        quit_key
    ).autorelease();
    app_menu.addItem_(quit_item);
    app_menu_item.setSubmenu_(app_menu);
}