#[macro_use]
extern crate gdnative;

mod astra;
mod godot_classes;
mod util;

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<godot_classes::AstraController>();
}

fn terminate(handle: *mut gdnative::sys::godot_gdnative_terminate_options) {
    godot_print!("terminate");

    unsafe {
        astra::terminate();
    }
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!(terminate);
