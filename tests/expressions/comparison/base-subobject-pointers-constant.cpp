// tidal-rules: CMP-8
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: not an integral constant expression
// tidal-error-edg: constant value
// tidal-note: Q-0387. No rule orders two base class subobjects of one object, so constant evaluation rejects the comparison. Clang and EDG reject. GCC gives a result.
struct Left { int a; };
struct Right { int b; };
struct Both : Left, Right { };
constexpr Both both{};
static_assert(static_cast<const void*>(static_cast<const Left*>(&both)) < static_cast<const void*>(static_cast<const Right*>(&both)) || true);
