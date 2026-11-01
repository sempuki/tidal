// tidal-rules: TSP-11
// tidal-expect: reject
// tidal-error-clang: specializes a template parameter with dependent type
// tidal-error-gcc: depends on a template parameter
// tidal-error-edg: the template argument list of the partial specialization includes a nontype argument whose type depends on a template parameter
// tidal-note: V has type T. The partial specialization fixes V to 1 and keeps T as a parameter.
template <class T, T V> struct Value { static constexpr int kind = 0; };
template <class T> struct Value<T, 1> { static constexpr int kind = 1; };
