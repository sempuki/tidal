// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-error-gcc-14: static assertion failed
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0361. A base that declares a deleted constructor keeps its end padding on GCC 16 and Clang. GCC 14 and EDG put the derived member into that padding.
struct NoDefault { NoDefault() = delete; int a; char b; };
struct NoCopy { NoCopy() = default; NoCopy(const NoCopy&) = delete; int a; char b; };
struct NoMove { NoMove() = default; NoMove(NoMove&&) = delete; int a; char b; };
template <class B> struct Next : B { char c; };
static_assert(sizeof(Next<NoDefault>) == 12);
static_assert(sizeof(Next<NoCopy>) == 12);
static_assert(sizeof(Next<NoMove>) == 12);
