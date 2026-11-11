// A generic lambda inside a function template whose parameter is auto.
// From views.cpp.
int apply(auto k) {
  auto f = [k](auto r) { return r * k; };
  return f(3);
}
int main() { return apply(2) == 6 ? 0 : 1; }
