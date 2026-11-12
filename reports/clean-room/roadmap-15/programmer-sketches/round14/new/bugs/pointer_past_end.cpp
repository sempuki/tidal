int main() {
  int a[2] = {1, 2};
  int* p = a + 3;  // two past the end
  return p - a == 3 ? 0 : 1;
}
