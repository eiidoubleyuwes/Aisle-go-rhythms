#This algorithm is supposed to be the modulos exponentiation algorithm
#It is supposed to take in three parameters, base, exponent, and mod, and return base^exponent mod
#The algorithm is supposed to be implemented using the divide and conquer strategy
#The algorithm is supposed to be implemented using recursion
#I have just converted my Rust code to Python

def mod_exponent(base, binary, modu):
  result = 1

  for bit in binary:
    result = result * result % modu

    if bit == 1:  
      result = (base * result) % modu

  return result

def main():
  base = input("Enter the base: ")
  exponent = input("Enter the exponent: ") 
  modu = input("Enter the mod: ")

  base = int(base)
  exponent = int(exponent)
  modu = int(modu)

  binary = []
  exp = exponent
  while exp > 0:
    binary.append(exp % 2)
    exp = exp // 2

  binary.reverse()

  print("The Binary of the exponent:", binary)

  print("The result of the mod-exponent algorithm is:", 
        mod_exponent(base, binary, modu))

main()

    
    
    

    
   