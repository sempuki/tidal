// tidal-rules: LAM-15 DEP-24
// tidal-expect: run
// tidal-note: Write suite W19, finding W19-2 (reports/clean-room/W19/report.md). Inside a function template, a call that names the template arguments of a lambda's call operator template on an object of dependent type uses template.
template <class F, class... Ts> constexpr int sum_sizes(F f) {
  return (f.template operator()<Ts>() + ... + 0);
}
int main() {
  auto size_of = []<class T>() { return static_cast<int>(sizeof(T)); };
  return sum_sizes<decltype(size_of), char, char>(size_of) == 2 ? 0 : 1;
}
