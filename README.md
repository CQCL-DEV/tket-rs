# tket-rs

## Building

The project uses conan to download and statically link the tket libraries, make sure you
have added a conan remote with tket available for your system (there may be instructions for
this on the tket repository). For current tket releases, you will need gcc 11.

And install conan with `pip install conan`. 
After that `cargo build --release` should do the job (note release is needed for
linking to work correctly). Make sure linking and loading is succesful with
`cargo test --release`. 

You will need a conan profile that matches the pattern
`tket-rs-{os}-{platform}`, e.g. `tket-rs-linux-x86_64`. So a command like

```
conan profile new tket-rs-linux-x86_64 --detect
```


should work. 

You might also need to run 

```
conan profile update settings.compiler.libcxx=libstdc++11 tket-rs-linux-x86_64
```


Now go check that profile, crucially you need to make sure compiler version is
11, compiler is gcc, and build_type is Release.


Here is an example: 
```
[settings]
os=Linux
os_build=Linux
arch=x86_64
arch_build=x86_64
compiler=gcc
compiler.version=11
compiler.libcxx=libstdc++11
build_type=Release
[options]
[build_requires]
[env]
```