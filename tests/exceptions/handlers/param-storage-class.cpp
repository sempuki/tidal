// tidal-rules: TR-12
// tidal-expect: reject
// tidal-error-clang: does not allow storage class
// tidal-error-gcc: expected type-specifier before .static
// tidal-error-edg: storage class may not be specified here
// tidal-note: The exception_decl production derives no storage class.
void g() { try { } catch (static int e) { } }
