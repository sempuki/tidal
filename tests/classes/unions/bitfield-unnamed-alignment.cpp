// tidal-rules: BF-9
// tidal-expect: accept
// tidal-note: x86-64. An unnamed bit-field moves the next bits as BF-9 gives, and does not change the alignment of the class.
struct WideGap { char a; long long : 3; };   // the unnamed bit-field does not raise the alignment
struct IntGap { char a; int : 0; char b; };      // b starts at the next 4-byte unit
struct EndGap { char a; int : 0; };              // the class still grows to the unit
static_assert(alignof(WideGap) == 1 && sizeof(WideGap) == 2);
static_assert(alignof(IntGap) == 1 && sizeof(IntGap) == 5);
static_assert(alignof(EndGap) == 1 && sizeof(EndGap) == 4);
