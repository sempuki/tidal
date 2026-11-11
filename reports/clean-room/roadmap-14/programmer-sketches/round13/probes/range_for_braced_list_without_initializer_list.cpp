int main() {
  int a = 1, b = 2, s = 0;
  for (int* p : {&a, &b}) s += *p;
  return s - 3;
}
