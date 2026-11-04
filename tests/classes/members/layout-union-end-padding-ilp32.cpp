// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-flags: -m32
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0384. -m32. The sizes are the same as on x86-64 (layout-union-end-padding.cpp). EDG keeps the end padding of every union.
union Plain { char c[5]; int i; };
union Built { Built() {} char c[5]; int i; };
struct PlainThen { [[no_unique_address]] Plain u; char x; };
struct BuiltThen { [[no_unique_address]] Built u; char x; };
static_assert(sizeof(PlainThen) == 12);
static_assert(sizeof(BuiltThen) == 8);
