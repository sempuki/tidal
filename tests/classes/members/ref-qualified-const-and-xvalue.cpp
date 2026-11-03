// tidal-rules: CLS-6
// tidal-expect: run
// tidal-note: The overloads are const & and &&. A const lvalue selects const &, and an xvalue from a cast selects &&.
struct S {
  int which() const & { return 1; }
  int which() && { return 2; }
};
int main() {
  const S c{};
  S s;
  return c.which() == 1 && s.which() == 1 && static_cast<S&&>(s).which() == 2 ? 0 : 1;
}
