// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: cannot combine with previous .static. declaration specifier
// tidal-error-gcc: extern. specifier conflicts with .static
// tidal-error-edg: more than one storage class
static extern int x;
