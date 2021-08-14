local moonalloy = {{}}


-- Load the FFI module
local ffi = require("ffi")

-- Define the structs and functions to search for in the shared library
ffi.cdef[[

typedef struct {
  int len;
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

typedef struct {
  int rows;
  int cols;
  array_t* arrays;
} matrix_t;

matrix_t* matrix_zeros(int rows, int cols);
matrix_t* matrix_ones(int rows, int cols);
matrix_t* matrix_identity(int len);
void matrix_print(matrix_t *mat);
char* matrix_to_string(const matrix_t* mat);
matrix_t* matrix_add(const matrix_t *mat1, const matrix_t *mat2);
matrix_t* matrix_sub(const matrix_t *mat1, const matrix_t *mat2);
matrix_t* matrix_elem_mult(const matrix_t *mat1, const matrix_t *mat2);
matrix_t* matrix_transpose(const matrix_t *mat);
matrix_t* matrix_mult(const matrix_t *mat1, const matrix_t *mat2);
matrix_t* matrix_scalar(const matrix_t *mat, double scal);

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
  return rust_lib.array_dotp(self.array, self.array)
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
  return a:mult(b)
end

Array.__concat = function(a, b)
  return a:concat(b)
end

Array.__tostring = function(a)
  return tostring(a.array)
end


-- Matrix metatype
local mat

local mat_mt = {
  __index = mat,
  __add = function(m, n) return rust_lib.matrix_add(m, n) end,
  __sub = function(m, n) return rust_lib.matrix_sub(m, n) end,
  __mul = function(m, n) return rust_lib.matrix_mult(m, n) end,
  __tostring = function(m) return ffi.string(rust_lib.matrix_to_string(m)) end,
}

mat = ffi.metatype("matrix_t", mat_mt)

local function is_valid_matrix(t)
  local len = nil

  for i = 1, #t do
    if (len ~= nil) then
      if (len ~= #t[i]) then
        return false
      end
    else
      len = #t[i]
    end
  end

  return true
end

local function new_matrix(t) 
  local slice = {}
  local cols

  assert(is_valid_matrix(t) == true, "Invalid table - Cannot be converted into a matrix: Asymmetric dimensions.")

  for i = 1, #t do
    slice[i] = new_array(t[i])
    cols = #slice[i]
  end

  local length = "array_t[" .. #t .. "]"
  local new = mat(#slice, cols, ffi.new(length, slice))
  return new
end

-- Matrix Wrapper class
Matrix = { rows = 0, cols = 0, matrix = nil }
Matrix.__index = Matrix

setmetatable(Matrix, {
    __call = function (cls, ...)
      return cls.new(...)
    end,
  })

function Matrix.new(t)
  local self = setmetatable({}, Matrix)

  self.rows = #t
  self.cols = #t[1]
  self.matrix = new_matrix(t)

  return self
end

function Matrix:from(rows, cols, matrix)
  assert(rows > 0, "ERROR: Number of rows must be positive.")
  assert(cols > 0, "ERROR: Number of columns must be positive.")

  setmetatable(self, Matrix)
  self.matrix = matrix
  self.rows = rows
  self.cols = cols

  return self
end

function Matrix:print()
  rust_lib.matrix_print(self.matrix)
end

function Matrix:zeros(rows, cols)
  assert(rows > 0, "ERROR: Number of rows must be positive.")
  assert(cols > 0, "ERROR: Number of columns must be positive.")

  local result = setmetatable({}, Matrix)
  result.matrix = rust_lib.matrix_zeros(rows, cols)
  result.rows = self.rows
  result.cols = self.cols
  result.__index = result

  return result
end

function Matrix:ones(rows, cols)
  assert(rows > 0, "ERROR: Number of rows must be positive.")
  assert(cols > 0, "ERROR: Number of columns must be positive.")

  local result = setmetatable({}, Matrix)
  result.matrix = rust_lib.matrix_ones(rows, cols)
  result.rows = rows
  result.cols = cols
  result.__index = result

  return result
end

function Matrix:identity(len)
  assert(len > 0, "ERROR: Cannot create an identity matrix smaller than 1x1.")

  local result = setmetatable({}, Matrix)
  result.matrix = rust_lib.matrix_identity(len)
  result.rows = len
  result.cols = len
  result.__index = result

  return result
end

function Matrix:add(other)
  assert(self.rows == other.rows, "ERROR: Matrices differ in number of rows.")
  assert(self.cols == other.cols, "ERROR: Matrices differ in number of columns.")

  local result = setmetatable({}, Matrix)
  result.matrix = self.matrix + other.matrix
  result.rows = self.rows
  result.cols = self.cols
  result.__index = result

  return result
end

function Matrix:sub(other)
  assert(self.rows == other.rows, "ERROR: Matrices differ in number of rows.")
  assert(self.cols == other.cols, "ERROR: Matrices differ in number of columns.")

  local result = setmetatable({}, Matrix)
  result.matrix = self.matrix - other.matrix
  result.rows = self.rows
  result.cols = self.cols
  result.__index = result

  return result
end

function Matrix:elem_mult(other)
  assert(self.rows == other.rows, "ERROR: Matrices differ in number of rows.")
  assert(self.cols == other.cols, "ERROR: Matrices differ in number of columns.")

  local result = setmetatable({}, Matrix)
  result.matrix = rust_lib.matrix_elem_mult(self.matrix, other.matrix)
  result.rows = self.rows
  result.cols = self.cols
  result.__index = result

  return result
end

function Matrix:transpose()
  local result = setmetatable({}, Matrix)
  result.matrix = rust_lib.matrix_transpose(self.matrix)
  result.rows = self.rows
  result.cols = self.cols
  result.__index = result

  return result
end

function Matrix:mult(other)
  assert(self.rows == other.cols, "ERROR: Cannot multiply matrices - Incompatible dimensions.")

  local result = setmetatable({}, Matrix)
  result.matrix = self.matrix * other.matrix
  result.rows = other.rows
  result.cols = self.cols
  result.__index = result

  return result
end

function Matrix:scalar(scal)
  local result = setmetatable({}, Matrix)
  result.matrix = rust_lib.matrix_scalar(self.matrix, scal)
  result.rows = self.rows
  result.cols = self.cols
  result.__index = result

  return result
end

Matrix.__tostring = function(m)
  return tostring(m.matrix)
end

Matrix.__add = function(m, n)
  return m:add(n)
end

Matrix.__sub = function(m, n)
  return m:sub(n)
end

Matrix.__mul = function(m, n)
  return m:mult(n)
end


-- Test for the entire module
function moonalloy.test_array()
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
  print("multed = a * a2 = ", multed)

  print("a:size() = ", #a)

  print("a:sum() = ", rust_lib.array_sum(a))

  local conc = a .. a2
  print("conc = ", conc)

  print("tostring(a) = ", tostring(a))

  local add_again = added + a2
  print("add_again = added + a2 = ", add_again)

  a = a + added
  print("a = a + added = ", a)

  local mult_again = multed * a
  print("mult_again = multed * added = ", mult_again)

  -- For debugging
  print("Success!")
end

function moonalloy.test_matrix() 
  local m = new_matrix({{1.0, 2.0}, {3.0, 4.0}})
  print("m = ", m)

  local m2 = new_matrix({{2.0, 3.0}, {5.0, 8.0}})
  print("m2 = ", m2)

  local added = m + m2
  print("added = ", added)

  local t = rust_lib.matrix_transpose(m)
  print("t = ", t)

  local multed = m * m
  print("multed = ", multed)

  m = m + added
  print("m = m + added = ", m)

  -- For debugging
  print("Success!")
end

-- Return moonalloy to create the module (can now be used with "require")
return moonalloy
