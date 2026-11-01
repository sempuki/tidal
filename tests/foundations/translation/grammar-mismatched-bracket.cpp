// tidal-rules: TR-12
// tidal-expect: reject
// tidal-error-clang: expected .\]
// tidal-error-gcc: expected .\]. before .\). token
// tidal-error-edg: expected a .\]
// tidal-note: The array bound opens with [ and ends with ), so no production derives the declarator.
int table[3) = {1, 2, 3};
