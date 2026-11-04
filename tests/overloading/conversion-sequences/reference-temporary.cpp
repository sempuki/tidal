// tidal-rules: ICS-11
// tidal-expect: accept
// tidal-note: A binding to a temporary has the sequence of the conversion to the referenced type: short to int is a promotion, short to long a conversion. int to const long& and int to long tie, so the second argument decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
short small = 1;
One take(const long&);
Two take(const int&);
static_assert(same<decltype(take(small)), Two>);
namespace first { One put(const long&, int); Two put(long, long); }
namespace second { One put(const long&, long); Two put(long, int); }
static_assert(same<decltype(first::put(1, 0)), One>);
static_assert(same<decltype(second::put(1, 0)), Two>);
