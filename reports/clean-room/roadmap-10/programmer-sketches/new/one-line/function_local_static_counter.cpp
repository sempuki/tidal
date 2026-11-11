// A function-local static, initialized once on the first call.
int next_id() { static int id = 100; return id++; }
int main() {
  int a = next_id();
  int b = next_id();
  return a == 100 && b == 101 ? 0 : 1;
}
