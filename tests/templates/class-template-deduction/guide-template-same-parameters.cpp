// tidal-rules: CTAD-12 CTAD-24
// tidal-expect: reject
// tidal-error-clang: ambiguous deduction for template arguments
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: more than one instance of constructor "Box" matches
// tidal-note: Every compiler accepts the two guide templates with the same parameters and different results. A use that both can take is ambiguous (Q-0280).
template <class T> struct Box { constexpr Box(T) {} };
template <class T> Box(T*) -> Box<long>;
template <class T> Box(T*) -> Box<short>;
int i;
Box b(&i);
