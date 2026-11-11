template <class M> struct traits { using cls = int; };
template <auto V> int call(typename traits<decltype(V)>::cls x) { return x + V; }
int main() {
  return call<3>(4) == 7 ? 0 : 1;
}
