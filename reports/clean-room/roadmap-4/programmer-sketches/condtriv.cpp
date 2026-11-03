// A conditionally trivial optional.
template <class T> struct Opt {
  union { T v; char none; };
  bool has = false;
  constexpr Opt() : none() {}
  constexpr Opt(T x) : v(x), has(true) {}
  ~Opt() requires requires { requires __is_trivially_destructible(T); } = default;
  ~Opt() { if (has) v.~T(); }
};
struct D { ~D() {} };
template <class T> concept TrivDtor = requires { requires __is_trivially_destructible(T); };
static_assert(TrivDtor<Opt<int>>);
static_assert(!TrivDtor<Opt<D>>);
int main() { Opt<int> a(3); Opt<D> b; return a.v == 3 ? 0 : 1; }
