// tidal-rules: TPL-23 OPR-6 OPR-7
// tidal-expect: run
// tidal-note: Member operators of a class template, defined after the class body and used with operator syntax.
template <int Scale> struct Fixed {
  long long raw;
  bool operator<(Fixed other) const;
  Fixed operator+(Fixed other) const;
};
template <int Scale> bool Fixed<Scale>::operator<(Fixed other) const { return raw < other.raw; }
template <int Scale> Fixed<Scale> Fixed<Scale>::operator+(Fixed other) const { return {raw + other.raw}; }
int main() {
  Fixed<100> a{3}, b{5};
  Fixed<100> c = a + b;
  return a < b && !(b < a) && c.raw == 8 ? 0 : 1;
}
