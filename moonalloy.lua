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
array_t* concat(const array_t* arr1, const array_t *arr2);
char* to_string(const array_t* arr);

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
  __concat = function(a, b) return rust_lib.concat(a, b) end,
  __tostring = function(a) return ffi.string(rust_lib.to_string(a)) end,
  __index = arr,
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

  setmetatable(self, Array)
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

function Array:concat(other)
  local array = Array:from(self.array .. other.array, self.len + other.len)
  return array
end

function Array:tostring()
  return tostring(self)
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

Array.__concat = function(a, b)
  return a:concat(b)
end

Array.__tostring = function(a)
  return tostring(a.array)
end


function moonalloy.test_module()
  -- Create a table
  local arg = {1.0, 2.0, 3.0}

  local a = new_array(arg)
  print("a = ", a)

  local a2 = new_array({2.0, 3.0, 5.0})
  print("a2 = ", a2)

  local added = a + a2
  print("added = ", added)

  print("a = ", a)
  print("a2 = ", a2)

  local multed = a * a2
  print("multed = ", multed)

  print("a:size() = ", #a)

  print("a:sum() = ", rust_lib.sum(a))

  local conc = a .. a2
  print("conc = ", conc)

  print("tostring(a) = ", tostring(a))

  -- For debugging
  print("Success!")
end


-- Return moonalloy to create the module (can now be used with "require")
return moonalloy
