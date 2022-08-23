import sys
read = sys.stdin.read
L1, R1, L2, R2 = list(map(int, read().split()))
print(max(0, min(R1, R2) - max(L1, L2)))
