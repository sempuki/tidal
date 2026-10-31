// tidal-rules: CEV-1
// tidal-expect: reject
// tidal-error-clang: call to consteval function .sq. is not a constant expression
// tidal-error-gcc: call to consteval function .sq\(v\). is not a constant expression
// tidal-error-edg: call to consteval function .sq. did not produce a valid constant expression
consteval int sq(int v) { return v * v; }
constexpr int wrap(int v) { return sq(v); }
int main(int argc, char**) { return wrap(argc); }
