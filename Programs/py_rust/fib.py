import sys
import time
RUNS = 100


def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n -1) + fibonacci(n - 2)

def main():
    
    n = int(input("Please input value for n: "))
    print(f"fibonacci({n}) = ")
    
    sum_time = 0
    
    for i in range(RUNS):
        start_time = time.time()
        fibonacci(n)
        end_time = time.time()
        time_taken = end_time - start_time
        sum_time += time_taken
        
    avg_time = sum_time / RUNS
    print(f"\nPython us per call: {avg_time * 1_000_000:.5f} us")
    print(f"\nPython ms per call: {avg_time * 1_000:.5f} ms")
    
    
if __name__ == "__main__":
    main()