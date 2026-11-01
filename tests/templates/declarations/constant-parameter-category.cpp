// tidal-rules: TPL-9
// tidal-expect: accept
// tidal-note: decltype((N)) shows the value category: T for a prvalue, T& for an lvalue.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct Point { int x; int y; };
int object = 0;
template <int N> constexpr bool int_is_prvalue = same<decltype((N)), int>;
template <int& R> constexpr bool ref_is_lvalue = same<decltype((R)), int&>;
template <Point P> constexpr bool class_is_const_lvalue = same<decltype((P)), const Point&>;
static_assert(int_is_prvalue<1>);
static_assert(ref_is_lvalue<object>);
static_assert(class_is_const_lvalue<Point{1, 2}>);
