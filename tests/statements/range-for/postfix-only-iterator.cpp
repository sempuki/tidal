// tidal-rules: RFOR-7
// tidal-expect: reject
// tidal-error-clang: cannot increment value of type .It
// tidal-error-gcc: no match for .operator\+\+.
// tidal-error-edg: no operator .\+\+. matches these operands
// tidal-note: Gap review 2026-10-30.
struct It {
  int v;
  int operator*() const { return v; }
  It operator++(int) { It old = *this; ++v; return old; }
  bool operator!=(const It& o) const { return v != o.v; }
};
struct R { It begin() { return {0}; } It end() { return {3}; } };
void f() { for (int x : R{}) (void)x; }
