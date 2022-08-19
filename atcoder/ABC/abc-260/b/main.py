import sys
readline = sys.stdin.readline

N, X, Y, Z = map(int, readline().split())
A = list(map(int, readline().split()))
B = list(map(int, readline().split()))

s = list((i + 1, A[i], B[i]) for i in range(N))
s.sort(key=lambda p: (-p[1], p[0]))

s[X:] = sorted(s[X:], key=lambda p: (-p[2], p[0]))
s[X + Y:] = sorted(s[X + Y:], key=lambda p: (-(p[1] + p[2]), p[0]))
s[:X + Y + Z] = sorted(s[:X + Y + Z], key=lambda p: p[0])

for p in s[:X + Y + Z]:
    print(p[0])
