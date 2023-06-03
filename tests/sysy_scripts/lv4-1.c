int main() {
  const int x = 1 + 1, z = 0 || 1;
  const int y = !x;
  return x - y + z;
} // for level 4.1