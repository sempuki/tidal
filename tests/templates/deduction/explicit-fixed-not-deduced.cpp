// tidal-rules: DED-3 DED-35
// tidal-expect: reject
// tidal-error-clang: deduced conflicting types for parameter
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: Without explicit arguments, the conversion function takes no part in deduction, and T gets two values.
struct Meter { constexpr operator int() const { return 5; } };
template <class T> constexpr T add(T a, T b) { return a + b; }
int n = add(Meter{}, 1);
