// tidal-rules: CPT-33
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: Anything does not use its second parameter. Clang and EDG still substitute the extra argument, and the requirement fails. GCC skips it and finds the requirement satisfied (Q-0287).
template <class T, class U> concept Anything = true;
template <class T> concept Typed = requires (T t) { { t } -> Anything<typename T::type>; };
static_assert(!Typed<int>);
