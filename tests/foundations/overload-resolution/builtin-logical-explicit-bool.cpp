// tidal-rules: OVL-9
// tidal-expect: accept
// tidal-note: The built-in candidates of !, &&, and || take the explicit conversion function to bool. No other candidate exists.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct Flag { explicit operator bool() const; };
Flag f;
static_assert(same<decltype(!f), bool>);
static_assert(same<decltype(f && f), bool>);
static_assert(same<decltype(f || f), bool>);
