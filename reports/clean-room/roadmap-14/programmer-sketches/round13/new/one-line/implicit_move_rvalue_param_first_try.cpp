// return of an rvalue reference parameter by name gives an xvalue.
struct Box { int v; };
Box&& pass(Box&& b) { return static_cast<Box&&>(b); }
decltype(auto) forward_it(Box&& b) { return b; }
template <class T> constexpr bool is_rref = false;
template <class T> constexpr bool is_rref<T&&> = true;
int main() {
  static_assert(!is_rref<decltype(forward_it(Box{1}))>);
  Box b = pass(Box{2});
  return b.v - 2;
}
