int main() {
  const int y = 10 - 3 * 2;
  int x = 10 * y, z = y - 1;
  x = (x + 1) % z;

  // for level 4.2
  return x % y;
}