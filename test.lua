local moonalloy = require("moonalloy")


function test_Array()
  local arg = {1.0, 2.0, 3.0}

  local a = Array.new(arg)
  print("a = ", a)

  local b = Array.new({2.0, 3.0, 5.0})
  print("b = ", b)

  local c = Array({3.0, 5.0, 8.0})
  print("c = ", c)

  local scal = c:scalar(3.0)
  print("scal = ", scal)

  local added = a + b
  print("added = ", added)

  local subbed = a - c
  print("subbed = ", subbed)

  local conc = added .. subbed
  print("conc = ", conc)

  a = a + c
  print("a = ", a)
  
  local z = Array:zeros(3)
  print("z = ", z)

  local o = Array:ones(3)
  print("o = ", o)

  print("success!")
end

function test_Matrix() 
  local m = Matrix:new({{1.0, 2.0}, {3.0, 4.0}})
  print("m = ", m)

  local o = Matrix:ones(3, 3)
  print("o = ", o)

  local z = Matrix:zeros(2, 2)
  print("z = ", z)

  local i = Matrix:identity(5)
  print("i = ", i)

  print("success!")
end

-- Test imported 

-- moonalloy.test_array()
-- moonalloy.test_matrix()
-- test_Array()
test_Matrix()

