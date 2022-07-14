# tket-rs

## Building

The project uses conan to download and statically link the tket libraries, make sure you
have added a conan remote with tket available (there may be instructions for
this on the tket repository). And install conan with `pip install conan`. 
After that `cargo build` should do the job.