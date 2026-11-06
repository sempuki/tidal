// I3-5: does an int* lvalue bind const int* const& directly (same object),
// and does an int* prvalue bind through a temporary?
int x = 0;
int* p = &x;
int* make() { return &x; }
int f(const int* const&) { return 1; }
int f(const int*&&) { return 2; }
int main() {
  const int* const& r = p;
  bool direct = static_cast<const void*>(&r) == static_cast<const void*>(&p);
  int lv = f(p);        // lvalue: only the const& form binds
  int pr = f(make());   // prvalue: both bind a temporary; rvalue reference wins (ICS-28)
  __builtin_printf("direct=%d lvalue=%d prvalue=%d\n", direct, lv, pr);
  return 0;
}
