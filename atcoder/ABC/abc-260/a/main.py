import sys

read = sys.stdin.read
S = read()

if S[0] == S[1] and S[1] == S[2]:
    print('-1')
    exit()

if S[0] == S[1]:
    print(S[2])
elif S[0] == S[2]:
    print(S[1])
else:
    print(S[0])
