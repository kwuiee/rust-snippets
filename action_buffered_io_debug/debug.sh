# Trace syscall write, unbuffered
strace --trace=write target/release/02_buffered
# Trace syscall write, buffered
strace --trace=write target/release/02_buffered
# Trace syscall read, unbuffered
strace --trace=read target/release/03_unbuffered_json
# Trace syscall read, buffered
strace --trace=read target/release/04_buffered_json

# Trace syscall unbuffered read
# The strace option --decode-fds=path tells us to which file the descriptor 3 refers (in this case, /tmp/simple.json)
# which can be helpful to know what part of the program is at fault.
strace --decode-fds=path --trace=read target/release/03_unbuffered_json 2>&1 | \
	awk '/= 1$/'
