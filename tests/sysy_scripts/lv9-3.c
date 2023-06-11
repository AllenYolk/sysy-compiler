int f(int arr[][3+2]) {
  return arr[1][2];
}

int main() {
  int arr[2][3][5] = {0, 1, 2, 3, 4, {5, 6}, {7}};
  return f(arr[0]);
}