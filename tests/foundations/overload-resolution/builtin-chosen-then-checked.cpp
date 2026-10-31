// tidal-rules: OVL-11
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator\+.
// tidal-error-edg: no operator .\+. matches these operands
// tidal-note: The candidate for int* and the difference type wins. Holder converts to int*, and 1.5 keeps its type, so the built-in + gets a pointer and a double.
struct Holder { operator int*() const; };
auto r = Holder{} + 1.5;
