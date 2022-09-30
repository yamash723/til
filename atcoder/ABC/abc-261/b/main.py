n = int(input())
metrics = [input() for _ in range(n)]

for i in range(n):
    for j in range(i + 1, n):
        if ((metrics[i][j] == 'W' and metrics[j][i] != 'L') or
            (metrics[i][j] == 'L' and metrics[j][i] != 'W') or
            (metrics[i][j] == 'D' and metrics[j][i] != 'D')):
            print('incorrect')
            exit(0)

print('correct')
