// tidal-rules: ICS-8 CONV-17
// tidal-expect: reject
// tidal-error-clang: ambiguous conversion from derived class
// tidal-error-gcc: is an ambiguous base
// tidal-error-edg: base class .Base. is ambiguous
// tidal-note: An ambiguous base does not stop the sequence, so take(Base*) is chosen and the conversion fails (CONV-17).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base {};
struct Left : Base {};
struct Right : Base {};
struct Joined : Left, Right {};
Joined* j = nullptr;
One take(Base*);
Two take(...);
auto r = take(j);
