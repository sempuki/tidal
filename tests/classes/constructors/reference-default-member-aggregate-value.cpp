// tidal-rules: CTOR-17
// tidal-expect: run
// tidal-note: Q-0084. S is an aggregate whose default member initializer would bind r to a temporary. The only initialization gives r a value from the braces, so no constructor uses the default member initializer, and CTOR-17 does not reject the class.
int value = 1;
struct S {
  const int& r = 5;
};
S s{value};
int main() { return s.r == 1 && &s.r == &value ? 0 : 1; }
