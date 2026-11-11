// tidal-rules: DED-24
// tidal-expect: accept
// tidal-note: Gap review RV6-B-3. A qualified name whose member has the wrong kind is a substitution failure: a value where a type is needed, a type where a value is needed, and a name that is not a template after template. Each call selects the fallback on every compiler.
struct HasValue { static const int x = 1; };
struct HasType { using x = int; };
template <int> struct Slot {};
template <class T> constexpr int as_type(int, typename T::x* = nullptr) { return 1; }
template <class T> constexpr int as_type(long) { return 2; }
template <class T> constexpr int as_value(int, Slot<T::x>* = nullptr) { return 1; }
template <class T> constexpr int as_value(long) { return 2; }
template <class T> constexpr int as_template(int, typename T::template x<int>* = nullptr) { return 1; }
template <class T> constexpr int as_template(long) { return 2; }
static_assert(as_type<HasValue>(0) == 2);
static_assert(as_value<HasType>(0) == 2);
static_assert(as_template<HasType>(0) == 2);
