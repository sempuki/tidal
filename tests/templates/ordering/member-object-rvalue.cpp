// tidal-rules: ORD-6 ORD-15
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-expect-edg: accept
// tidal-error-clang: is ambiguous
// tidal-error-gcc: ambiguous overload
// tidal-note: The member has no ref-qualifier, and the other template's first parameter is X&&. The member's object parameter is then X&&, both pairs are the same, and neither template is more specialized. Clang 18 and EDG give the member X& and choose it (Q-0214).
struct One {}; struct Two {};
struct X { template <class T> One operator+(T); };
template <class T> Two operator+(X&&, T);
void use() { X{} + 1; }
