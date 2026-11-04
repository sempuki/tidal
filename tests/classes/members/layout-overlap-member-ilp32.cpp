// tidal-rules: CLS-31 CLS-34 ATTR-9 CLS-35
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. The same choices as on x86-64.
struct Empty {};
struct Plain { int a; char b; };
struct Built { Built() {} int a; char b; };
struct Beside { int value; [[no_unique_address]] Empty tag; };
struct Two { [[no_unique_address]] Empty first; [[no_unique_address]] Empty second; int value; };
struct Only { [[no_unique_address]] Empty first; [[no_unique_address]] Empty second; };
struct PlainThen { [[no_unique_address]] Plain part; char c; };
struct BuiltThen { [[no_unique_address]] Built part; char c; };
static_assert(sizeof(Beside) == sizeof(int) && sizeof(Two) == sizeof(int) && sizeof(Only) == 2);
static_assert(sizeof(PlainThen) == 12 && sizeof(BuiltThen) == 8);
