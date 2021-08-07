local moonalloy = require("moonalloy")


function test_Array()
  local arg = {1.0, 2.0, 3.0}

  local a = Array.new(arg)
  print("a = ", a)

  local b = Array.new({2.0, 3.0, 5.0})
  print("b = ", b)

  local c = Array({3.0, 5.0, 8.0})
  print("c = ", c)

  local added = a + b
  print("added = ", added)

  local subbed = a - c
  print("subbed = ", subbed)

  local conc = added .. subbed
  print("conc = ", conc)

  a = a + c
  print("a = ", a)

  print("Success!")
end

-- Test imported 

-- moonalloy.test_module()
test_Array()

