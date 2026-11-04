// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0384. x86-64. The end-padding conditions of CLS-35 apply to a union as to a class. Clang and GCC lend the padding of a union with a user-provided constructor or destructor, a private member, or a default member initializer, and keep the padding of a plain union. EDG keeps the end padding of every union.
union Plain { char c[5]; int i; };
union Built { Built() {} char c[5]; int i; };
union Ends { ~Ends() {} char c[5]; int i; };
union Hidden { char c[5]; int i; private: int j; };
union Starts { char c[5]; int i = 0; };
struct PlainThen { [[no_unique_address]] Plain u; char x; };
struct BuiltThen { [[no_unique_address]] Built u; char x; };
struct EndsThen { [[no_unique_address]] Ends u; char x; };
struct HiddenThen { [[no_unique_address]] Hidden u; char x; };
struct StartsThen { [[no_unique_address]] Starts u; char x; };
static_assert(sizeof(PlainThen) == 12);
static_assert(sizeof(BuiltThen) == 8 && sizeof(EndsThen) == 8);
static_assert(sizeof(HiddenThen) == 8 && sizeof(StartsThen) == 8);
