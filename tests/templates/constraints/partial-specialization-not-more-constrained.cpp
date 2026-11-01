// tidal-rules: CNS-36
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: not more specialized than the primary template
// tidal-error-gcc: not more constrained than the primary template
// tidal-note: Neither A nor B subsumes the other. EDG accepts the partial specialization (Q-0292).
template <class T> concept A = sizeof(T) > 1;
template <class T> concept B = sizeof(T) < 100;
template <A T> struct Kind {};
template <B T> struct Kind<T> {};
