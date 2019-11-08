#[macro_use]
extern crate gdnative;
mod godot_classes;
mod util;

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<godot_classes::AstraController>();
}

fn terminate(_handle: *mut gdnative::sys::godot_gdnative_terminate_options) {}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!(terminate);
