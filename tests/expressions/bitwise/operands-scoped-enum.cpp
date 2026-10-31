// tidal-rules: BIT-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*Flags
// tidal-error-gcc: no match for .operator\|.
// tidal-error-edg: no operator .\|. matches these operands
enum class Flags { A = 1, B = 2 };
auto r = Flags::A | Flags::B;
