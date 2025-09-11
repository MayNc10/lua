print ("Hello, World!")
for key,value in pairs(io) do
  print(key, type(value))
end

print()
print(io.input)
io.input("sources/io_test.txt")
print(io.input)