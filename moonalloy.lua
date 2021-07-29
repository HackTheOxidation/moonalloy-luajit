local ffi = require("ffi")

ffi.cdef[[

typedef struct {
  int len;
  double *arr;
} array_t;

void print(array_t *arr1);

double sum(array_t *arr);
array_t* add(array_t *arr1, array_t *arr2);
array_t* mult(array_t *arr1, array_t *arr2);
double dotp(array_t *arr1, array_t *arr2);

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
local len = "double[" .. #arg .. "]"
local ar = arr(3, ffi.new(len, arg))
local ar2 = arr(3, ffi.new(len, arg))
print(rust_lib.sum(ar))

local result = ar + ar2
rust_lib.print(result)

local another = rust_lib.mult(result, ar)
rust_lib.print(another)

local dot = rust_lib.dotp(ar2, another)
print(dot)

print("Success!")
