// tidal-rules: AUTO-22
// tidal-expect: reject
// tidal-error-clang: deduced as .int. in declaration of .* deduced as .double
// tidal-error-gcc: inconsistent deduction for .auto
// tidal-error-edg: type is .double. for this entity, but was previously implied
auto a = 1, b = 2.0;
