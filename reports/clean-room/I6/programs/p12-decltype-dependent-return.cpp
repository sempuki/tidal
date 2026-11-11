// DEP-1, DEP-2: decltype of a type-dependent expression in a trailing return type.
struct List { long size() const { return 3; } };
template <class T> auto count(const T& t) -> decltype(t.size()) { return t.size(); }
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(count(List{})), long>);
int main() { return count(List{}) == 3 ? 0 : 1; }
