// tidal-rules: CLS-35 CLS-34 CLS-31
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. The sizes are the same as on x86-64 (layout-empty-member-placement.cpp), because each class holds only int and char members.
struct Empty {};
struct Beside { int value; [[no_unique_address]] Empty tag; };
struct Between { char c; [[no_unique_address]] Empty tag; int v; };
struct Pair { [[no_unique_address]] Empty first; int v; [[no_unique_address]] Empty second; };
struct Small { [[no_unique_address]] Empty first; char c; [[no_unique_address]] Empty second; };
struct AfterPlain { Empty plain; char c; [[no_unique_address]] Empty tag; };
// first 0, d 0, data size 8, second 8, size 12 (8 + 1, rounded up to 4)
struct Wide { [[no_unique_address]] Empty first; double d; [[no_unique_address]] Empty second; };
static_assert(sizeof(Beside) == 4 && sizeof(Between) == 8 && sizeof(Pair) == 8);
static_assert(sizeof(Small) == 2 && sizeof(AfterPlain) == 3 && sizeof(Wide) == 12);
