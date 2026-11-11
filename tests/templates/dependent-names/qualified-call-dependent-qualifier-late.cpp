// tidal-rules: DEP-40 DEP-7
// tidal-expect: accept
// tidal-note: Gap review RV7-B-7. The qualifier Lib<T> depends on T, so the call Lib<T>::pick(value) is looked up at the instantiation. It finds the explicit specialization Lib<int>, which comes after the template. Every compiler accepts (writer probe roadmap-11/writer-probes/rv7-points.json).
template <class T> struct Lib { static constexpr int pick(T) { return 1; } };
template <class T> constexpr int call(T value) { return Lib<T>::pick(value); }
template <> struct Lib<int> { static constexpr int pick(int) { return 2; } };
static_assert(call(0) == 2);
static_assert(call('a') == 1);
