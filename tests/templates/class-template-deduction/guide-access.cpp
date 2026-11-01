// tidal-rules: CTAD-12
// tidal-expect: reject
// tidal-error-clang: deduction guide has different access from the corresponding member template
// tidal-error-gcc: must have the same access as
// tidal-error-edg: must be declared with the same
// tidal-note: The member template In is private and its guide is public.
class Outer {
  template <class T> struct In { In(T) {} };
public:
  template <class T> In(T*) -> In<T>;
};
