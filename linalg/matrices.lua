local matrices = {{}}
require('linalg/arrays')


-- Load the FFI module
local ffi = require("ffi")

-- Define the structs and functions to search for in the shared library
ffi.cdef[[


// Matrix
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

function new_matrix(t)
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
  if (type(m) == "number") then
    return n:scalar(m)
  elseif (type(n) == "number") then
    return m:scalar(n)
  else
    return m:mult(n)
  end
end

return matrices
