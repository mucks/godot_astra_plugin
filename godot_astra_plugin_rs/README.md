# Godot Plugin for Orbbec Astra SDK

Godot Plugin for the [Orbbec Astra Sdk](https://orbbec3d.com/develop/)

## Prerequisites to use this library

* Download and install the [Orbbec Astra Sdk](https://orbbec3d.com/develop/)
* make sure you have these envs in your .profile

```bash
# adjust astra home to your astra_sdk path
export ASTRA_HOME=$HOME/astra
export ASTRA_SDK_INCLUDE=$ASTRA_HOME/include
export ASTRA_SDK_LIB=$ASTRA_HOME/lib
# this is so that rust executables know where to find the astra libs
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$ASTRA_SDK_LIB
```

* add "source ~/.profile" to your .bashrc
* run godot from the terminal
