// tidal-rules: SHF-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*E
// tidal-error-gcc: no match for .operator<<.
// tidal-error-edg: no operator .<<. matches these operands
enum class E { A = 1 };
auto r = E::A << 1;
