template <class... Fs> struct Overload : Fs... { using Fs::operator()...; };
int main() {
  Overload o{[](int x) { return x; }, [](const auto&) { return 1; }};
  return o(0);
}
