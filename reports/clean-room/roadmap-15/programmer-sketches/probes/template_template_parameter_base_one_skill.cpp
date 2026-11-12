template <class S> struct Add { friend constexpr S operator+(const S& a, const S&) { return a; } };
template <class T, template <class> class Skill>
struct Strong : Skill<Strong<T, Skill>> { T v{}; };
using M = Strong<int, Add>;
template <class S> concept addable = requires(const S& a) { a + a; };
static_assert(addable<M>);
int main() { return 0; }
