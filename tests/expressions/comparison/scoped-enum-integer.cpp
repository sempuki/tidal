// tidal-rules: CMP-23
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator<.
// tidal-error-edg: no operator .<. matches these operands
enum class Level { low, high };
bool b = Level::low < 1;
