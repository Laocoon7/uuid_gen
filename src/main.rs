use clipboard::{ClipboardContext, ClipboardProvider};
use uuid::Uuid;

fn main() {
    // Create a clipboard context
    let mut ctx: ClipboardContext =
        ClipboardProvider::new().expect("Failed to create clipboard context");

    // Your string to copy to the clipboard
    let text_to_copy = format!("{}", Uuid::new_v4());

    // Copy the string to the clipboard
    ctx.set_contents(text_to_copy.to_owned())
        .expect("Failed to set clipboard contents");
}