struct C { int r; };
struct S { short x; C c; };
template <class T> int first(const T& t) {
  if constexpr (requires { T{1, 2, 3}; }) {
    auto& [a, b, c] = t;
    return a;
  } else {
    auto& [a, b] = t;
    return a;
  }
}
int main() { S s{0, {1}}; return first(s); }
