// tidal-rules: AUTO-22
// tidal-expect: reject
// tidal-error-clang: deduced as .int. in declaration of .* deduced as .double
// tidal-error-gcc: inconsistent deduction for .auto
// tidal-error-edg: type is .double. for this entity, but was previously implied
// tidal-note: Each declarator is a pointer. The first deduces int for auto and the second deduces double.
int i = 0;
double d = 0;
auto *a = &i, *b = &d;
