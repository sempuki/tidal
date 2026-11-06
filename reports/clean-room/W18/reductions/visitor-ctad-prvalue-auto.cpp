template <class... Fs> struct Overload : Fs... { using Fs::operator()...; };
int main() {
  auto o = Overload{[](int x) { return x; }, [](double) { return 1; }};
  return o(0);
}
