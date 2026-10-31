// tidal-rules: CAST-8
// tidal-expect: reject
// tidal-error-gcc: reinterpret_cast. is not a constant expression
int x;
constexpr long* p = reinterpret_cast<long*>(&x);
