int half_add(int x, int y) {
  return x / 2 + y;
}

void ff(int x) {
  int y = x % 2;
  if (x > 0) {
    int z = 5;
  } else {
    ff(x - 2);
  }
}

int gg(int a, int b, int c, int d, int e, int f, int g, int h, int i, int j) {
  int xx = a + b + c + d + e + f + g + h + i + j;
  return xx;
}

int main() {
  ff(3 + 5 * 2);
  return half_add(10, 1);
}