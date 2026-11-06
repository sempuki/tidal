// PACK-27: two packs of different lengths in one expansion are a substitution failure.
template <class... A> struct List {};
template <class X, class Y> struct Pair {};
template <class... A, class... B> int zip(List<A...>, List<B...>, List<Pair<A, B>...>* = nullptr) { return 1; }
int zip(...) { return 2; }
int main() { return zip(List<int>{}, List<int, int>{}) == 2 && zip(List<int>{}, List<char>{}) == 1 ? 0 : 1; }
