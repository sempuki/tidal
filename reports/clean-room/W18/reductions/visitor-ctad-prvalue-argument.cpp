template <class... Fs> struct Overload : Fs... { using Fs::operator()...; };
template <class V> int apply(V&& v) { return v(0); }
int main() {
  return apply(Overload{[](int x) { return x; }, [](double) { return 1; }});
}
