int main() {
  int a = 2;
  if (a) {
    a = a + 1;
    int b = 2 * a;
    if (a) {b = b + 1;
    if (b < 0) b = -b; }
    else b = b % 4;
    a = a + b;
  } else {
    a = 0;
  }  // fafafafa
  return a; // for level 6.1
}