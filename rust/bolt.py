import sys, ctypes
from ctypes import c_void_p, c_char_p, c_uint32, c_int32, Structure, POINTER


class BoltS(Structure):
    pass


# Load bolt
lib = ctypes.cdll.LoadLibrary('interface/target/release/libbolt.dylib')
lib.bolt_new.restype = POINTER(BoltS)
lib.bolt_free.argtypes = (POINTER(BoltS), )
lib.bolt_get_cwd.argtypes = (POINTER(BoltS), c_int32)
lib.bolt_get_cwd.restype = (c_void_p)


class Bolt:
    def __init__(self):
        self.obj = lib.bolt_new()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.bolt_free(self.obj)

    def get_cwd(self, id):
        ptr = lib.bolt_get_cwd(self.obj, id)
        return ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')


if __name__ == "__main__":
    bolt = Bolt()
    print("Bolt CWD: " + bolt.get_cwd(0))
