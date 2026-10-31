// tidal-rules: CV-2
// tidal-expect: reject
// tidal-error-clang: duplicate .const
// tidal-error-gcc: duplicate .const
// tidal-error-edg: type qualifier specified more than once
const const int x = 1;
