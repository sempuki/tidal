// tidal-rules: TSP-25
// tidal-expect: accept
// tidal-note: The template is constexpr and the explicit specialization for char is not, so size(char{}) is not a constant. The concept tests that without an error.
template <class T> constexpr int size(T) { return 0; }
template <> int size(char) { return 1; }
template <int> struct Fixed {};
template <class T> concept constant_size = requires { typename Fixed<size(T{})>; };
static_assert(constant_size<int>);
static_assert(!constant_size<char>);
