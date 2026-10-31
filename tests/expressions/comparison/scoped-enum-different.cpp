// tidal-rules: CMP-10
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*E.*F
// tidal-error-gcc: no match for .operator==.
// tidal-error-edg: no operator .==. matches these operands
enum class E { A };
enum class F { A };
bool b = E::A == F::A;
