int main() {
  int i = 0, x = 1;
  if (x < 10)
    while(i < 3)
      if (x < 10)
        x = x * x + 1;
      else
        i = i + 1;
  return x;
}