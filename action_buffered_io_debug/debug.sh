# Trace syscall write, unbuffered
strace --trace=write target/release/02_buffered
# Trace syscall write, buffered
strace --trace=write target/release/02_buffered
# Trace syscall read, unbuffered
strace --trace=read target/release/03_unbuffered_json
# Trace syscall read, buffered
strace --trace=read target/release/04_buffered_json
