// tidal-rules: CAST-8
// tidal-expect: reject
int x;
constexpr long* p = reinterpret_cast<long*>(&x);
