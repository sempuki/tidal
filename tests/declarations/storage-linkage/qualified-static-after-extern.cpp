// tidal-rules: LNK-10 NS-6
// tidal-expect: reject
// tidal-error-clang: static declaration of .count. follows non-static declaration
// tidal-error-gcc: was declared .extern. and later .static.
// tidal-error-edg: external/internal linkage conflict
namespace limits {
extern int count;
}
static int limits::count = 1;
