// tidal-rules: CLS-33 CLS-31 CLS-11
// tidal-expect: accept
// tidal-note: A class whose only members are empty [[no_unique_address]] members, or unnamed bit-fields of width 0, is an empty class. As a standard-layout base it takes no bytes (CLS-33). An unnamed bit-field of nonzero width makes the class not empty, so the base takes its byte.
struct Empty {};
struct OnlyMarked { [[no_unique_address]] Empty e; };
struct OnlyZeroWidth { int : 0; };
struct OnlyBits { int : 3; };
struct FromMarked : OnlyMarked { int x; };
struct FromZeroWidth : OnlyZeroWidth { int x; };
struct FromBits : OnlyBits { int x; };
static_assert(sizeof(OnlyMarked) == 1 && sizeof(OnlyZeroWidth) == 1 && sizeof(OnlyBits) == 1);
static_assert(sizeof(FromMarked) == sizeof(int));
static_assert(sizeof(FromZeroWidth) == sizeof(int));
static_assert(sizeof(FromBits) == 2 * sizeof(int));
