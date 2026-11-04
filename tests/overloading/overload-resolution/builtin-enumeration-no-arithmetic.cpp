// tidal-rules: OPR-15 OVL-9
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator\+.
// tidal-error-edg: no operator .\+. matches these operands
// tidal-note: A scoped enumeration does not promote, so no built-in + candidate accepts it.
enum class Mode { a, b };
struct ToMode { operator Mode() const; };
auto r = ToMode{} + ToMode{};
