#This is just the converted Rust code to Python
import math
import sys

def is_prime(number):
  for i in range(2, int(math.sqrt(number))+1):
    if number % i == 0:
      return False
  return True

def main():

  print("Enter the limit: ")
  limit = int(input())

  numbers = list(range(2, limit+1)) 
  primes = []

  for number in numbers:
    if is_prime(number):
      primes.append(number)
      print(number)

  print(primes)

if __name__ == '__main__':
  main()
