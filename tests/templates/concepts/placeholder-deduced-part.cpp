// tidal-rules: CPT-23
// tidal-expect: accept
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> concept Int = same<T, int>;
template <class T> concept LvalueRef = same<T, T&>;
int n = 0;
const Int auto& ref = n;
Int auto* ptr = &n;
Int auto&& moved = n + 1;
LvalueRef auto&& forwarded = n;
LvalueRef decltype(auto) named = (n);
