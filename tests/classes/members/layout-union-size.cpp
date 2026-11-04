// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-note: x86-64. A union has the size of its largest member, rounded up to the largest alignment of its members, and at least 1. Its data size is the size of its largest member, so a class that does not keep its end padding still fills the union.
union Five { char c[5]; int i; };
union Wide { char c; double d; };
union Nothing {};
struct Lends { Lends() {} int i; char c; };
// one member of size 8 whose class lends its padding
union HoldsLends { Lends l; };
struct After { [[no_unique_address]] HoldsLends u; char x; };
static_assert(sizeof(Five) == 8 && alignof(Five) == 4);
static_assert(sizeof(Wide) == 8 && alignof(Wide) == 8);
static_assert(sizeof(Nothing) == 1);
static_assert(sizeof(After) == 12);
