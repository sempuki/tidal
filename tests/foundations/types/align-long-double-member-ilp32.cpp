// tidal-rules: TYP-2
// tidal-expect: accept
// tidal-flags: -m32
struct Wide { char tag; long double value; };
static_assert(sizeof(Wide) == 16 && alignof(Wide) == 4);
