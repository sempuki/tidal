// tidal-rules: ADD-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*E
// tidal-error-gcc: no match for .operator\+.
// tidal-error-edg: no operator .\+. matches these operands
// tidal-note: Gap review 2026-10-31 (third).
enum class E { one = 1 };
int arr[4];
int* p = arr + E::one;
