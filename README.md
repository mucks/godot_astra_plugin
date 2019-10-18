gdnative needs java headers

sudo cp -r /usr/lib/jvm/java-8-openjdk/include/* /usr/include/
sudo cp -r /usr/lib/jvm/java-8-openjdk/include/linux/* /usr/include/

persee requires target ndk version 22 (android 5.1)

${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 22 --arch arm64 --install-dir NDK/arm64
${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 22 --arch arm --install-dir NDK/arm
${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 22 --arch x86 --install-dir NDK/x86


export requires sdk aar to work