// A member function of a class template that exists only for some arguments.
template <class T> struct Wrap {
  T v;
  T twice() const requires requires(T t) { t + t; } { return v + v; }
};
struct Opaque {};
template <class W> concept HasTwice = requires(W w) { w.twice(); };
static_assert(HasTwice<Wrap<int>> && !HasTwice<Wrap<Opaque>>);
int main() { Wrap<int> w{4}; Wrap<Opaque> o{}; (void)o; return w.twice() == 8 ? 0 : 1; }
