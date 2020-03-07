from cffi import FFI

ffi = FFI()
# wirte header
ffi.cdef("""
    uint64_t fibonacci(uint32_t);
""")
 
C = ffi.dlopen("target/release/libfibonacci.so")

print(C.fibonacci(100))
