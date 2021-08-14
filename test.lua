local moonalloy = require("moonalloy")


function test_Array()
  local arg = {1.0, 2.0, 3.0}

  local a = Array.new(arg)
  print("a = ", a, ", ", a.len)

  local b = Array.new({2.0, 3.0, 5.0})
  print("b = ", b)

  local c = Array({3.0, 5.0, 8.0})
  print("c = ", c)

  local d = Array({1.0, 1.0, 1.0})
  print("d = ", d)

  local scal = c:scalar(3.0)
  print("scal = ", scal)

  local added = a + b
  print("added = ", added)

  local subbed = a - c
  print("subbed = ", subbed)

  local conc = added .. subbed
  print("conc = ", conc)

  local a = a + c
  print("a = ", a)
  
  local z = Array:zeros(3)
  print("z = ", z)

  local o = Array:ones(3)
  print("o = ", o)

  local multed = b * c
  print("multed = ", multed)

  local again = multed * added
  print("again = ", again)

  print("Success!")
end

function test_Matrix() 
  local m = Matrix({{1.0, 2.0}, {3.0, 4.0}})
  print("m = ", m)

  local m2 = Matrix({{2.0, 3.0}, {5.0, 8.0}})
  print("m2 = ", m2)

  local o = Matrix:ones(3, 3)
  print("o = ", o)

  local z = Matrix:zeros(2, 2)
  print("z = ", z)

  local i = Matrix:identity(5)
  print("i = ", i)

  local added = m + m2
  print("added = ", added)

  local subbed = added - m2
  print("subbed = ", subbed)

  local elem_multed = m:elem_mult(m2)
  print("elem_multed = ", elem_multed)

  local t = m:transpose()
  print("t = ", t)

  local multed = m * m
  print("multed = ", multed)

  local scal = m:scalar(2.0)
  print("scal = ", scal)

  print("Success!")
end


-- moonalloy.test_array()
moonalloy.test_matrix()
-- test_Array()
-- test_Matrix()

