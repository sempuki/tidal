// tidal-rules: RFOR-7
// tidal-expect: reject
// tidal-error-clang: cannot use incomplete type .* as a range
// tidal-error-gcc: has incomplete type
// tidal-error-edg: array of unknown size or incomplete type
struct Incomplete;
extern Incomplete items[];
void f() {
  for (auto& v : items) (void)v;
}
