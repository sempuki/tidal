// An rvalue reference parameter named in a return statement is an xvalue:
// a move-only value moves out, and decltype(auto) gives Only&&.
struct Only {
  int v;
  Only(int x) : v(x) {}
  Only(Only&&) = default;
  Only(const Only&) = delete;
};
Only take(Only&& o) { return o; }
decltype(auto) pass(Only&& o) { return o; }
template <class T> constexpr bool is_rref = false;
template <class T> constexpr bool is_rref<T&&> = true;
int main() {
  static_assert(is_rref<decltype(pass(Only{1}))>);
  Only a = take(Only{2});
  Only b = pass(Only{3});
  return a.v + b.v - 5;
}
