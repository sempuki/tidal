// tidal-rules: LNK-8
// tidal-expect: reject
// tidal-error-clang: static declaration of .g. follows non-static declaration
// tidal-error-gcc: was declared .extern. and later .static
// tidal-error-edg: external/internal linkage conflict
int g();
static int g();
