def main():
    n, m, x, t, d = map(int, input().split())

    if x <= m:
        print(t)
        return

    print(t - ((x - m) * d))


if __name__ == "__main__":
    main()
