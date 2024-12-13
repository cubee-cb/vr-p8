
# build
cargo build
sudo setcap cap_sys_ptrace+ep target/debug/vr-p8

# run
target/debug/vr-p8
