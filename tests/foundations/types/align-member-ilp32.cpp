// tidal-rules: TYP-2
// tidal-expect: accept
// tidal-flags: -m32
struct Pair { char tag; long long value; };
struct Real { char tag; double value; };
static_assert(sizeof(Pair) == 12 && sizeof(Real) == 12);
