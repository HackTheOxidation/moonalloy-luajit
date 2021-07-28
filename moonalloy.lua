local ffi = require("ffi")

ffi.cdef[[

typedef struct {
  int len;
  double *arr;
} array_t;

double sum(const array_t *arr);
const array_t* add(const array_t *arr1, const array_t *arr2);
void print(const array_t *arr1);
]]

rust_lib = ffi.load("./moonalloy/target/debug/libmoonalloy.so")

local arr
local mt = {
  __add = function(a, b) return rust_lib.add(a, b) end,
  __len = function(a) return a.len end,
  __index = {},
}

arr = ffi.metatype("array_t", mt)

local arg = {1.0, 2.0, 3.0}
local ar = arr(3, ffi.new("double[3]", arg))
local ar2 = arr(3, ffi.new("double[3]", arg))
print(rust_lib.sum(ar))
rust_lib.print(ar)
local result = ar + ar2
 rust_lib.print(result)
print("Success!")
