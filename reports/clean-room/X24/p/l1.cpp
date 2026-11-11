// X24 probe l1: one label name in a lambda body and in its enclosing
// function (JMP-5, JMP-14: separate functions).
int f() {
  int n = 0;
outer:
  for (int i = 0; i < 3; ++i) {
    auto g = [&] {
    outer:
      for (int j = 0; j < 3; ++j) { if (j == 1) goto done; ++n; }
    done:;
    };
    g();
    if (i == 1) goto outer_end;
  }
outer_end:
  return n;
}
int main() { return f() == 2 ? 0 : 1; }
