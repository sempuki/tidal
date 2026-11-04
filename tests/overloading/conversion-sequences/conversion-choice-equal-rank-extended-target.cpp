// tidal-rules: ICS-44
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: invalid suffix .f32. on floating constant
// tidal-note: PRIM-22: Clang has no f32 suffix. The target is the extended type of the rank of float. float converts to it by conversion rank from the same rank, and double from another rank, so only the source of equal rank decides.
using Float32 = decltype(1.0f32);
struct Source {
  constexpr operator float() const { return 1; }
  constexpr operator double() const { return 2; }
};
constexpr Float32 f = Source{};
static_assert(f == 1);
