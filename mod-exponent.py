#This algorithm is supposed to be the modulos exponentiation algorithm
#It is supposed to take in three parameters, a, b, and c, and return a^b mod c
#The algorithm is supposed to be implemented using the divide and conquer strategy
#The algorithm is supposed to be implemented using recursion

def mod_exponent(base, exponent, mod):
    #Allow user to input the base, exponent, and mod
    base = int(input("Enter the base: "))
    exponent = int(input("Enter the exponent: "))
    mod = int(input("Enter the mod: "))
    
   