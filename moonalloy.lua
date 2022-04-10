local moonalloy = {{}}

require('linalg/arrays')
require('linalg/matrices')


-- Load the FFI module
local ffi = require("ffi")

-- Load the shared library from '.so'-file
local rust_lib = ffi.load("/usr/local/lib/libmoonalloy.so")

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
  print("\nSuccess!\n\n")
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
  print("\nSuccess!\n\n")
end

-- Return moonalloy to create the module (can now be used with "require")
return moonalloy
