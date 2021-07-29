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
array_t* sub(array_t *arr1, array_t *arr2);
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
  __sub = function(a, b) return rust_lib.sub(a, b) end,
  __len = function(a) return a.len end,
  __mul = function(a, b) return rust_lib.mult(a, b) end,
  __index = {},
}

-- Creates the metatype with functions and operators
arr = ffi.metatype("array_t", mt)

function new_array(t) 
  local length = "double[" .. #t .. "]"
  local new = arr(#t, ffi.new(length, t))
  return new
end

-- Create a table and format its length
local arg = {1.0, 2.0, 3.0}
local len = "double[" .. #arg .. "]"

-- Create two Array metatypes and test the sum() function
local ar = arr(#arg, ffi.new(len, arg))
rust_lib.print(ar)
local ar2 = new_array({2.0, 3.0, 5.0})
print(rust_lib.sum(ar))

-- Tests the len function
print(#ar)

-- Test the + operator and the underlying add() function
local result = ar + ar2
rust_lib.print(result)

local ar3 = new_array({2.3, 5.1, 8.2})
rust_lib.print(ar3)

-- Test the - operator and the underlying sub() function
local sub = ar - ar2
rust_lib.print(sub)

-- Test the * operator and the underlying mult() function
local another = result * ar
rust_lib.print(another)

-- Test the dotp() function
local dot = rust_lib.dotp(ar2, another)
print(dot)

-- For debugging
print("Success!")

-- Return moonalloy to create the module (can now be used with "require")
return moonalloy
