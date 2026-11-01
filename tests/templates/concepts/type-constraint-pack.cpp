// tidal-rules: CPT-19
// tidal-expect: accept
template <class T> concept Big = sizeof(T) > 1;
template <Big... Ts> constexpr int count = sizeof...(Ts);
template <class... T> concept Counts = requires { count<T...>; };
static_assert(Counts<> && Counts<int, double> && !Counts<int, char>);
