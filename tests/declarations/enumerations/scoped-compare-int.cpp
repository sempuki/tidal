// tidal-rules: ENUM-16
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression \(.E. and .int.\)
// tidal-error-gcc: no match for .operator==
// tidal-error-edg: no operator .==. matches these operands
enum class E { A };
bool b = E::A == 0;
