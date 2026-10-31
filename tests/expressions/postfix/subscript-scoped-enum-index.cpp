// tidal-rules: PFX-2
// tidal-expect: reject
// tidal-error-clang: array subscript is not an integer
// tidal-error-gcc: array subscript is not an integer
// tidal-error-edg: no operator .\[\]. matches these operands
enum class Index { First };
int a[2];
int f() { return a[Index::First]; }
