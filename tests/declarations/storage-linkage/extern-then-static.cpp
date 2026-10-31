// tidal-rules: LNK-8
// tidal-expect: reject
// tidal-error-clang: static declaration of .y. follows non-static declaration
// tidal-error-gcc: was declared .extern. and later .static
// tidal-error-edg: external/internal linkage conflict
extern int y;
static int y;
