// Unevaluated operands: the comma as a test of validity.
template <class T> auto has_plus(T t, int) -> decltype(t + t, true) { return true; }
template <class T> bool has_plus(T, long) { return false; }
struct NoPlus {};
template <class T> concept Addable = requires(T t) { (t + t, 0); };
int x = 0;
static_assert(sizeof((x, 'c')) == 1);
static_assert(noexcept((x, 0)));
static_assert(Addable<int> && !Addable<NoPlus>);
int main() { return has_plus(1, 0) && !has_plus(NoPlus{}, 0) ? 0 : 1; }
