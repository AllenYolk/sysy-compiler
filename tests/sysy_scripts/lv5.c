void main() {
  int a = 1;
  {
    a = 2;
    ;
    int a = 3;
  }
  ;
  a + 6 - 1;
  return ;
  //return a;
}