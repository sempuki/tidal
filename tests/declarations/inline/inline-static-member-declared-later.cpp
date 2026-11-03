// tidal-rules: INL-8
// tidal-expect: reject
// tidal-error-clang: incomplete type .Later
// tidal-error-gcc: has incomplete type
// tidal-error-edg: incomplete type .Later. is not allowed
// tidal-note: The type of the inline static member is a class that is declared before and defined after the member.
struct Later;
struct S { static inline Later item{}; };
struct Later {};
