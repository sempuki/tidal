// tidal-rules: NEW-20
// tidal-expect: reject
// tidal-error-clang: const-qualified argument|no matching .operator new
// tidal-error-gcc: invalid conversion from .const void\*. to .void\*
// tidal-error-edg: no instance of overloaded .*operator new
inline void* operator new(decltype(sizeof 0), void* place) noexcept { return place; }
struct Gauge { int level; };
void refill(const Gauge* slot) { new (slot) Gauge{3}; }
