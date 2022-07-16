from main import main
import sys
from io import StringIO
import unittest


class TestClass(unittest.TestCase):
    def assertIO(self, input, output):
        stdout, stdin = sys.stdout, sys.stdin
        sys.stdout, sys.stdin = StringIO(), StringIO(input)
        main()
        sys.stdout.seek(0)
        out = sys.stdout.read()[:-1]
        sys.stdout, sys.stdin = stdout, stdin
        self.assertEqual(out, output)

    def test_入力例_1(self):
        input = """38 20 17 168 3"""
        output = """168"""
        self.assertIO(input, output)

    def test_入力例_2(self):
        input = """1 0 1 3 2"""
        output = """1"""
        self.assertIO(input, output)

    def test_入力例_3(self):
        input = """100 10 100 180 1"""
        output = """90"""
        self.assertIO(input, output)


if __name__ == "__main__":
    unittest.main()
