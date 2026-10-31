// tidal-rules: TYN-10
// tidal-expect: reject
// tidal-error-clang: incomplete type .struct Hidden
// tidal-error-gcc: incomplete type .struct n::Hidden|cannot convert .Hidden\*. to .n::Hidden\*
// tidal-error-edg: incompatible with parameter of type .n::Hidden
// tidal-note: The parameter declares n::Hidden, a different class from ::Hidden.
namespace n { void use(struct Hidden* p); }
struct Hidden { int v = 1; };
namespace n { void use(struct Hidden* p) { p->v = 2; } }
int main() { Hidden h; n::use(&h); return h.v == 2 ? 0 : 1; }
