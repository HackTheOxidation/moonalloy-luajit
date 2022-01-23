local arrays = {{}}


-- Load the FFI module
local ffi = require("ffi")

-- Define the structs and functions to search for in the shared library
ffi.cdef[[

// Array
typedef struct {
  size_t len;
  double *arr;
} array_t;

void array_print(const array_t *arr1);
double array_sum(array_t *arr);
array_t* array_scalar(const array_t *arr1, double scal);
array_t* array_add(const array_t *arr1, const array_t *arr2);
array_t* array_sub(const array_t *arr1, const array_t *arr2);
array_t* array_mult(const array_t *arr1, const array_t *arr2);
double array_dotp(const array_t *arr1, const array_t *arr2);
array_t* array_concat(const array_t* arr1, const array_t *arr2);
char* array_to_string(const array_t* arr);
array_t* array_zeros(int len);
array_t* array_ones(int len);
]]


-- Load the shared library from '.so'-file
local rust_lib = ffi.load("./moonalloy/target/debug/libmoonalloy.so")


-- Metatype for Array
local arr

-- Functions and operator overloads for the metatype
local arr_mt = {
  __add = function(a, b) return rust_lib.array_add(a, b) end, __sub = function(a, b) return rust_lib.array_sub(a, b) end,
  __len = function(a) return a.len end,
  __mul = function(a, b) return rust_lib.array_mult(a, b) end,
  __concat = function(a, b) return rust_lib.array_concat(a, b) end,
  __tostring = function(a) return ffi.string(rust_lib.array_to_string(a)) end,
  __index = arr,
}

-- Creates the metatype with functions and operators
arr = ffi.metatype("array_t", arr_mt)

function new_array(t)
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
  rust_lib.array_print(self.array)
end

function Array:scalar(scal)
  local array = rust_lib.array_scalar(self.array, scal)

  local result = setmetatable({}, Array)
  result.array = array
  result.len = self.len
  result.__index = result
  return result
end

function Array:add(other)
  assert(self.len == other.len, "ERROR: Arrays must have equal lengths.")

  local array = self.array + other.array

  local result = setmetatable({}, Array)
  result.array = array
  result.len = self.len
  result.__index = result
  return result
end

function Array:sub(other)
  assert(self.len == other.len, "ERROR: Arrays must have equal lengths.")

  local array = self.array - other.array

  local result = setmetatable({}, Array)
  result.array = array
  result.len = self.len
  result.__index = result
  return result
end

function Array:sum()
  return rust_lib.array_sum(self.array)
end

function Array:size()
  return #self.array
end

function Array:mult(other)
  assert(self.len == other.len, "ERROR: Arrays must have equal lengths.")

  local array = self.array * other.array

  local result = setmetatable({}, Array)
  result.array = array
  result.len = self.len
  result.__index = result
  return result
end

function Array:dotp(other)
  return rust_lib.array_dotp(self.array, other.array)
end

function Array:concat(other)
  local array = self.array .. other.array

  local result = setmetatable({}, Array)
  result.array = array
  result.len = self.len + other.len
  result.__index = result
  return result
end

function Array:tostring()
  return tostring(self)
end

function Array:zeros(len)
  assert(len > 0, "ERROR: Length must be positive.")

  setmetatable(self, Array)
  self.array = rust_lib.array_zeros(len)
  self.len = len

  return self
end

function Array:ones(len)
  assert(len > 0, "ERROR: Length must be positive.")

  setmetatable(self, Array)
  self.array = rust_lib.array_ones(len)
  self.len = len

  return self
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
  if (type(a) == "number") then
    return b:scalar(a)
  elseif (type(b) == "number") then
    return a:scalar(b)
  else
    return a:mult(b)
  end
end

Array.__concat = function(a, b)
  return a:concat(b)
end

Array.__tostring = function(a)
  return tostring(a.array)
end

return arrays
