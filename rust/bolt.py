import sys, ctypes
from ctypes import c_void_p, c_char_p, c_uint32, c_int32, Structure, POINTER, c_size_t


class BoltS(Structure):
    pass


# Load bolt
lib = ctypes.cdll.LoadLibrary('interface/target/release/libbolt.dylib')
lib.bolt_new.restype = POINTER(BoltS)
lib.bolt_free.argtypes = (POINTER(BoltS), )
lib.bolt_get_cwd.argtypes = (POINTER(BoltS), c_int32)
lib.bolt_get_cwd.restype = (c_void_p)
lib.bolt_free_string.argtypes = (c_void_p, )
lib.bolt_get_listing.argtypes = (c_void_p, c_int32, c_int32, POINTER(c_int32), c_size_t)
lib.bolt_get_listing.restype = c_int32
lib.bolt_get_num_entries.argtypes = (c_void_p, c_int32)
lib.bolt_get_num_entries.restype = c_int32
lib.bolt_get_entry_name.argtypes = (POINTER(BoltS), c_int32, c_int32)
lib.bolt_get_entry_name.restype = (c_void_p)


class Bolt:
    def __init__(self):
        self.obj = lib.bolt_new()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.bolt_free(self.obj)

    def get_cwd(self, id):
        ptr = lib.bolt_get_cwd(self.obj, id)
        try:
            return ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
        finally:
            lib.bolt_free_string(ptr)

    def get_entry_name(self, id, entryId):
        ptr = lib.bolt_get_entry_name(self.obj, id, entryId)
        try:
            return ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
        finally:
            lib.bolt_free_string(ptr)

    def get_listing(self, amount, offset, id):
        buf = []
        for i in range(0, amount):
            buf.append(0)
        buf_type = c_int32 * len(buf)
        rBuf = buf_type(*buf)
        numRet = lib.bolt_get_listing(self.obj, id, offset, rBuf, len(buf))
        res = []
        for i in range(0, numRet):
            res.append(rBuf[i])
        return res

    def get_num_entries(self, id):
        return lib.bolt_get_num_entries(self.obj, id)


if __name__ == "__main__":
    bolt = Bolt()
    print("Bolt CWD: " + bolt.get_cwd(0))
    for i in bolt.get_listing(50, 0, 0):
        print(bolt.get_entry_name(0, i))
