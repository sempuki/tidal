// X24 probe l2: a break in a member function of a local class, inside a
// loop of the enclosing function (JMP-7 names only a lambda body).
void f() {
  for (int i = 0; i < 3; ++i) {
    struct S { void m() { break; } };
    S{}.m();
  }
}
int main() {}
