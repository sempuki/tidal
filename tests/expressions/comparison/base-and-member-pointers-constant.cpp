// tidal-rules: CMP-8
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: not an integral constant expression
// tidal-error-edg: constant value
// tidal-note: Q-0387. No rule orders a base class subobject and a member of one object, so constant evaluation rejects the comparison. Clang and EDG reject. GCC gives a result.
struct Base { int a; };
struct Holder : Base { int m; };
constexpr Holder holder{};
static_assert(static_cast<const void*>(static_cast<const Base*>(&holder)) < static_cast<const void*>(&holder.m) || true);
