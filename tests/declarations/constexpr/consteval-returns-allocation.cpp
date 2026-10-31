// tidal-rules: CEV-1
// tidal-expect: reject
// tidal-error-clang: call to consteval function .make. is not a constant expression
// tidal-error-gcc: result of .operator new
// tidal-error-edg: call to consteval function .make. did not produce a valid constant expression
// tidal-note: Gap review 2026-10-31.
consteval int* make() { return new int(1); }
int main() { int* p = make(); delete p; return 0; }
