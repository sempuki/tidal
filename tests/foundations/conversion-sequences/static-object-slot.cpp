// tidal-rules: ICS-14
// tidal-expect: accept
// tidal-note: The object slot of the static function ties with the identity of the other, the first arguments tie, and the second argument decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Tool { static One use(long, int); Two use(unsigned, long); };
struct Tool2 { static One use(long, long); Two use(unsigned, int); };
Tool t;
Tool2 t2;
static_assert(same<decltype(t.use(1, 0)), One>);
static_assert(same<decltype(t2.use(1, 0)), Two>);
