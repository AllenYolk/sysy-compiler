/*
int half_add(int x, int y) {
  return x / 2 + y;
}

void f(int x) {
  int y = x % 2;
  if (x > 0) {
    int z = 5;
  } else {
    f(x - 2);
  }
}

int main() {
  f(3 + 5 * 2);
  return half_add(10, 1);
}
*/

int half(int x) {
  return x / 2;
}

void f() {}

int main() {
  f();
  return half(10);
}