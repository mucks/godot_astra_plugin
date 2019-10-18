fn main() {
    println!("cargo:rustc-link-lib=dylib=astra_core_api");
    println!("cargo:rustc-link-lib=dylib=astra_core");
    println!("cargo:rustc-link-lib=dylib=astra");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:include=/usr/lib/jvm/java-8-openjdk/include");
    let path = std::env::current_dir()
        .unwrap()
        .join("./android/jni/armeabi-v7a/");
    println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
}
