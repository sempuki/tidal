template <auto V> int call(decltype(V) x) { return x + V; }
int main() {
  return call<3>(4) == 7 ? 0 : 1;
}
