// tidal-rules: CAST-7
// tidal-expect: reject
// tidal-error-clang: casts away qualifiers
// tidal-error-gcc: casts away qualifiers
// tidal-error-edg: cannot cast away const
const int c = 1;
long* p = reinterpret_cast<long*>(&c);
