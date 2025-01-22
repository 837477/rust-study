def sum_python():
    total = 0
    for i in range(1, 1_000_000_001):
        total += i
    return total

import time
start = time.time()
_result = sum_python()
end = time.time()
print(f"Elapsed time: {end - start}")