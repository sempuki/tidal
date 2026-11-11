// Capture-free lambdas converted with unary plus into a table.
int main() {
  int (*ops[])(int, int) = {+[](int a, int b) { return a + b; }, +[](int a, int b) { return a * b; }};
  return ops[0](2, 3) + ops[1](2, 3) == 11 ? 0 : 1;
}
