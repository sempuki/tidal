// tidal-rules: CPT-26 LIST-2
// tidal-expect: accept
// tidal-note: X17-8. int{t} narrows for a double or long long t, so the requirement does not hold for those types. The program stays valid.
template <class T>
constexpr bool braces_to_int = requires (T t) { int{t}; };

static_assert(!braces_to_int<double>);
static_assert(!braces_to_int<long long>);
static_assert(braces_to_int<int>);
static_assert(braces_to_int<short>);
static_assert(braces_to_int<char>);
