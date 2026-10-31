// tidal-rules: MUL-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*Scoped
// tidal-error-gcc: no match for .operator\*.
// tidal-error-edg: no operator .\*. matches these operands
enum class Scoped { One = 1 };
auto r = Scoped::One * 2;
