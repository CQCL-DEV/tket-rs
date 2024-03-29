# tket-rs

## Building

The project uses conan to download and statically link the tket libraries, make sure you
have added a conan remote with tket available for your system (there may be instructions for
this on the tket repository). For current tket releases, you will need gcc 11.
For bindgen and autocxx you need Clang.

And install conan with `pip install conan~=1.60`.
After that `cargo build --release` should do the job (note release is needed for
linking to work correctly). Make sure linking and loading is succesful with
`cargo test --release`. 

You will need a conan profile that matches the pattern
`tket-rs-{os}-{platform}`, e.g. `tket-rs-linux-x86_64`. So a command like

```bash
conan profile new tket-rs-linux-x86_64 --detect
```


should work. 

You might also need to run 

```bash
conan profile update settings.compiler.libcxx=libstdc++11 tket-rs-linux-x86_64
```


Now go check that profile, crucially you need to make sure compiler version is
11, compiler is gcc, and build_type is Release.


Here is an example for a linux profile: 
```toml
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

Finally, make sure to add the remote with the tket pre-compiled binaries:
```bash
conan remote add tket-libs https://quantinuumsw.jfrog.io/artifactory/api/conan/tket1-libs
```

## Troubleshooting

Compilation may fail for many reasons. Here are some things to try:

Environment variables:
* Bindgen needs Clang, so you may need to point to it with something like
`LIBCLANG_PATH="${LLVM_PATH}/lib"`
* Bindgen may still fail to find c++ headers, so you may need to also set
  `BINDGEN_EXTRA_CLANG_ARGS="-I${LLVM_PATH}/lib/clang/12.0.1/include/
  -L${LIBCLANG_PATH}"` (or other system header paths on the include)
* On Ubuntu this may do the trick: `BINDGEN_EXTRA_CLANG_ARGS="-I/usr/include/c++/11 -I/usr/include/x86_64-linux-gnu/c++/11"`
