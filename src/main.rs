use std::panic;
use std::process;
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
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            panic_hook(panic_info);
            process::exit(1);
        }));

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
    let one_sec = Duration::from_secs(1);

    thread::spawn(move || {
        let btn = button_ptr as id;
        loop {
            let activity = activity::get_activity();
            let title = create_string(&*activity);
            btn.setTitle_(title);
            thread::sleep(one_sec);
        }
    });
}

unsafe fn create_string(string: &str) -> id {
    return NSString::alloc(nil).init_str(string);
}