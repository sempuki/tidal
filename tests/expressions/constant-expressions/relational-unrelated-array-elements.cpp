// tidal-rules: CE-15
// tidal-expect: reject
// tidal-error-clang: comparison between .* has unspecified value
// tidal-error-gcc: >= \(& y\[0\]\)\). is not a constant expression
// tidal-error-edg: pointer values cannot be compared
// tidal-note: The pointers point to elements of two different arrays.
int x[2], y[2];
constexpr bool ge = &x[1] >= &y[0];
