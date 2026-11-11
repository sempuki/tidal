// X24 probe s1: a label on a loop that a goto also names, and a chain
// of labels with an attribute before the loop.
int f() {
  int n = 0;
first:
second:
  [[likely]] for (int i = 0; i < 3; ++i) {
    ++n;
  }
  if (n < 6) goto second;
  if (n < 9) goto first;
  return n;
}
int main() { return f() == 9 ? 0 : 1; }
