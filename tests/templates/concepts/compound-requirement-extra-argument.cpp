// tidal-rules: CPT-33
// tidal-expect: accept
// tidal-note: The extra argument T::type is not valid for int, so the requirement does not hold.
template <class T, class U> concept Both = sizeof(T) > 0 && sizeof(U) > 0;
template <class T> concept Typed = requires (T t) { { t } -> Both<typename T::type>; };
static_assert(!Typed<int>);
