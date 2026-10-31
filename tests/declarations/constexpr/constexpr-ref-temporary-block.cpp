// tidal-rules: CX-1
// tidal-expect: reject
// tidal-error-clang: constexpr variable .r. must be initialized by a constant expression
// tidal-error-gcc: anonymous>. is not a constant expression
// tidal-error-edg: must have a constant value
// tidal-note: Gap review 2026-10-31.
int main() { constexpr const int& r = 1; return r; }
