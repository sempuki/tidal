// Direct-initialization of a class from another class through a conversion
// function: V(a), static_cast<V>(a), and V v(a). Copy-initialization runs.
struct V { int n; V(int x) : n(x) {} };
struct S { int n = 0; operator V() const { return V(n); } };
int main() {
  S a{3};
  V v1 = V(a);
  V v2 = static_cast<V>(a);
  V v3(a);
  return v1.n + v2.n + v3.n == 9 ? 0 : 1;
}
