// tidal-rules: CAST-4
// tidal-expect: reject
// tidal-error-clang: const_cast from .* is not allowed
// tidal-error-gcc: invalid .const_cast. from type
// tidal-error-edg: cannot change the underlying type
const int c = 1;
long* p = const_cast<long*>(&c);
