local moonalloy = {{}}

-- Load the FFI module
local ffi = require("ffi")

-- Define the structs and functions to search for in the shared library
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

-- Load the shared library from '.so'-file
rust_lib = ffi.load("./moonalloy/target/debug/libmoonalloy.so")

-- Metatype for Array
local arr
-- Functions and operator overloads for the metatype
local mt = {
  __add = function(a, b) return rust_lib.add(a, b) end,
  __len = function(a) return a.len end,
  __index = {},
}

-- Creates the metatype with functions and operators
arr = ffi.metatype("array_t", mt)

-- Create an array and format its length
local arg = {1.0, 2.0, 3.0}
local len = "double[" .. #arg .. "]"

-- Create two Array metatypes and test the sum() function
local ar = arr(3, ffi.new(len, arg))
local ar2 = arr(3, ffi.new(len, arg))
print(rust_lib.sum(ar))

-- Tests the len function
print(#ar)

-- Test the + operator and the underlying add() function
local result = ar + ar2
rust_lib.print(result)

-- Test the * operator and the underlying mult() function
local another = rust_lib.mult(result, ar)
rust_lib.print(another)

-- Test the dotp function
local dot = rust_lib.dotp(ar2, another)
print(dot)

-- For debugging
print("Success!")

-- Return moonalloy to create the module (can now be used with "require")
return moonalloy
