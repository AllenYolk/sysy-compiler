int main() {
  int i = 0, x = 1;
  
  while (!i) 
    while (x < 3) {
      if (x % 2)
        x = x + 1;
      else
        continue;
      i = i + 1;
      break;
    }

  return x;
}