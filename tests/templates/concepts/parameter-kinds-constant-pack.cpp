// tidal-rules: CPT-2
// tidal-expect: accept
// tidal-note: The concepts take a placeholder constant parameter and a constant pack. A constraint expression can be a call or a fold expression.
template <class T> constexpr int size_of() { return sizeof(T); }
template <auto V> concept NonZero = V != 0;
template <int... N> concept AllPositive = ((N > 0) && ...);
template <class T> concept Small = size_of<T>() < 4;
static_assert(NonZero<'a'> && !NonZero<0L>);
static_assert(AllPositive<1, 2> && AllPositive<> && !AllPositive<1, -1>);
static_assert(Small<char> && !Small<long long>);
