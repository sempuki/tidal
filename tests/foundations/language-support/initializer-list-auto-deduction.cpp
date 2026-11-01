// tidal-rules: LSUP-1 AUTO-7
// tidal-expect: reject
// tidal-error-clang: layout not recognized|weird std::initializer_list
// tidal-error-gcc: does not match
// tidal-error-edg: too many initializer values
namespace std {
template <class E> class initializer_list {};
}
int main() {
  auto values = {1, 2};   // std::initializer_list<int>
  (void)values;
  return 0;
}
