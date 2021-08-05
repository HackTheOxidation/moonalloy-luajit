local moonalloy = {{}}


-- Load the FFI module
local ffi = require("ffi")

-- Define the structs and functions to search for in the shared library
ffi.cdef[[

typedef struct {
  int len;
  double *arr;
} array_t;

void print(const array_t *arr1);

double sum(array_t *arr);
array_t* add(const array_t *arr1, const array_t *arr2);
array_t* sub(const array_t *arr1, const array_t *arr2);
array_t* mult(const array_t *arr1, const array_t *arr2);
double dotp(const array_t *arr1, const array_t *arr2);

]]

-- Load the shared library from '.so'-file
local rust_lib = ffi.load("./moonalloy/target/debug/libmoonalloy.so")


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

local function new_array(t) 
  local length = "double[" .. #t .. "]"
  local new = arr(#t, ffi.new(length, t))
  return new
end


-- Array Wrapper class
Array = {array = nil, len = 0}
Array.__index = Array

setmetatable(Array, {
    __call = function (cls, ...)
      return cls.new(...)
    end,
  })

-- Create a new Array Wrapper Object
function Array.new(aTable)
  local self = setmetatable({}, Array)

  self.len = #aTable
  self.array = new_array(aTable)

  return self
end

-- print() method for Array Wrapper
function Array:print()
  rust_lib.print(self.array)
end

function Array:from(array, len)

  self.array = array
  self.len = len

  return self
end

function Array:add(other)
  local array = Array:from(rust_lib.add(self.array, other.array), self.len)
  return array
end

function Array:sub(other)
  local array = Array:from(self.array - other.array, self.len)
  return array
end

function Array:sum()
  return rust_lib.sum(self.array)
end

function Array:size()
  return #self.array
end

function Array:mult(other)
  local array = Array:from(self.array * other.array, self.len)
  return array
end

function Array:dotp(other)
  return rust_lib.dotp(self.array, self.array)
end

Array.__add = function(a, b)
  return a:add(b)
end

Array.__sub = function(a, b)
      return a:sub(b)
    end

Array.__len = function(a)
  return a:size()
end

Array.__mul = function(a, b)
  return a:mult(b)
end


function moonalloy.test_module()
  -- Create a table
  local arg = {1.0, 2.0, 3.0}

  local a = new_array(arg)
  print("a = ")
  rust_lib.print(a)

  local a2 = new_array({2.0, 3.0, 5.0})
  print("a2 = ")
  rust_lib.print(a2)

  print("added = ")
  local added = a + a2
  rust_lib.print(added)

  print("a = ")
  rust_lib.print(a)
  print("a2 = ")
  rust_lib.print(a2)

  local multed = a * a2
  print("multed = ")
  rust_lib.print(multed)

  print("a:size() = ", #a)

  print("a:sum() = ", rust_lib.sum(a))

  print("a = ")
  rust_lib.print(a)

  -- For debugging
  print("Success!")
end

function moonalloy.test_Array()
  local arg = {1.0, 2.0, 3.0}

  local a = Array.new(arg)
  print("a = ")
  a:print()

  local b = Array.new({2.0, 3.0, 5.0})
  print("b = ")
  b:print()

  local c = Array({3.0, 5.0, 8.0})
  print("c = ")
  c:print()

  local added = a:add(b)
  print("added = ")
  added:print()

  local added2 = a + b
  print("added2 = ")
  added2:print()

  local subbed = a - c
  print("subbed = ")
  subbed:print()

  print("Success!")
end

-- Return moonalloy to create the module (can now be used with "require")
return moonalloy
