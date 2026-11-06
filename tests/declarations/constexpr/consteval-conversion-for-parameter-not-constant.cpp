// tidal-rules: CEV-1
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Roadmap round 8, programmer reduction consteval_converting_constructor_not_constant.cpp. Q-0413. The implicit conversion of -1 to the parameter calls a consteval constructor that is not a constant expression for that value, so the call is rejected. Clang and GCC reject. EDG accepts.
void fail();

struct checked {
  int n;
  consteval checked(int v) : n(v) { if (v < 0) fail(); }
};

int take(checked c) { return c.n; }

int main() { return take(-1); }
