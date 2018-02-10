import sys, ctypes
from ctypes import c_char_p, c_uint32, Structure, POINTER


class BoltS(Structure):
    pass


# Load bolt
lib = ctypes.cdll.LoadLibrary('interface/target/release/libbolt.dylib')
lib.bolt_new.restype = POINTER(BoltS)
lib.bolt_free.argtypes = (POINTER(BoltS), )


class Bolt:
    def __init__(self):
        self.obj = lib.bolt_new()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.bolt_free(self.obj)


if __name__ == "__main__":
    test = Bolt()
