// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: does not allow storage class
// tidal-error-gcc: expected type-specifier before .static
// tidal-error-edg: storage class may not be specified here
void g() { try { } catch (static int e) { } }
