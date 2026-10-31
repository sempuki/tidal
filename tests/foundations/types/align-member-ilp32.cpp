// tidal-rules: TYP-2 TYP-12
// tidal-expect: accept
// tidal-flags: -m32
// tidal-expect-edg-7.0-ilp32: reject
// tidal-error-edg-7.0-ilp32: static assertion failed
// tidal-note: Q-0120. EDG's i686 configuration aligns long long to 8 bytes; GCC and Clang align it to 4.
struct Pair { char tag; long long value; };
struct Real { char tag; double value; };
static_assert(sizeof(Pair) == 12 && sizeof(Real) == 12);
