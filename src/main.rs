use std::thread;
use std::time::Duration;

use cocoa::appkit::{NSApp,
                    NSApplication,
                    NSApplicationActivationPolicyProhibited,
                    NSButton,
                    NSMenu,
                    NSStatusBar,
                    NSStatusItem,
                    NSVariableStatusItemLength};
use cocoa::base::{id, nil, selector};
use cocoa::foundation::NSString;

mod activity;

fn main() {
    unsafe {
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyProhibited);

        let status_item = NSStatusBar::systemStatusBar(nil).statusItemWithLength_(NSVariableStatusItemLength);

        create_menu(status_item);
        start_activity_update_thread(status_item);

        app.run();
    }
}

unsafe fn create_menu(status_item: id) {
    let menu = NSMenu::alloc(nil);
    status_item.setMenu_(menu);

    let title = create_string("Quit");
    let action = selector("terminate:");
    let no_key = create_string("");
    menu.addItemWithTitle_action_keyEquivalent(title, action, no_key);
}

unsafe fn start_activity_update_thread(status_item: id) {
    let button_ptr = status_item.button() as u64;
    let three_secs = Duration::from_secs(3);

    thread::spawn(move || {
        let btn = button_ptr as id;
        loop {
            let activity = activity::get_activity();
            let title = create_string(&*activity);
            btn.setTitle_(title);
            thread::sleep(three_secs);
        }
    });
}

unsafe fn create_string(string: &str) -> id {
    return NSString::alloc(nil).init_str(string);
}