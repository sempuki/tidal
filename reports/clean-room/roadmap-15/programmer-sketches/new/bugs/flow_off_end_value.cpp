int sign(int x) {
  if (x > 0) return 1;
  if (x < 0) return -1;
}  // no return for 0
int main() { return sign(0) + 1 == 1 ? 0 : 1; }
