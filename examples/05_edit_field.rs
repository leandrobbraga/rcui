use rcui::curses::*;
use rcui::*;

fn main() {
    Rcui::exec(Proxy::wrap(
        |field, rcui, event| {
            if let Event::KeyStroke(key) = event {
                match *key {
                    KEY_LEFT => field.left(),
                    KEY_RIGHT => field.right(),
                    KEY_DC => field.delete_front(),
                    KEY_BACKSPACE => field.delete_back(),
                    _ => {
                        if *key as u8 as char == '\n' {
                            rcui.quit()
                        } else {
                            field.handle_event(rcui, event)
                        }
                    }
                }
            }
        },
        EditField::new(),
    ))
}
