// Dispatch through an array of function pointers indexed by an operation code.
int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }
int mul(int a, int b) { return a * b; }
using Op = int (*)(int, int);
constexpr Op ops[] = {add, sub, &mul};
int main() {
  int r = 0;
  for (unsigned i = 0; i < sizeof ops / sizeof ops[0]; ++i) r += ops[i](6, 3);
  return r == 9 + 3 + 18 ? 0 : 1;
}
