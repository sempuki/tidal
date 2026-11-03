// tidal-rules: OVL-24
// tidal-expect: accept
// tidal-note: A function template with a requires-clause. For char the constraint fails, so the template is not viable although it would match exactly.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
template <class T> concept Wide = sizeof(T) >= 8;
template <class T> requires Wide<T> One put(T);
Two put(long double);
static_assert(same<decltype(put(1.0)), One>);
static_assert(same<decltype(put('a')), Two>);
