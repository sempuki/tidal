// tidal-rules: CLS-26 CLS-6
// tidal-expect: run
// tidal-expect-gcc-14: reject
// tidal-error-gcc-14: cannot be overloaded with
// tidal-note: Q-0027. One overload is const with the ref-qualifier &, and the other has no cv-qualifier and no ref-qualifier. The cv-qualifiers differ, so the two can be overloaded. A const object calls the first, and a plain object the second. GCC 14 rejects the pair, as it rejects every mix of overloads with and without a ref-qualifier.
struct S {
  int f() const & { return 1; }
  int f() { return 2; }
};
int main() {
  S s;
  const S& c = s;
  return c.f() == 1 && s.f() == 2 ? 0 : 1;
}
