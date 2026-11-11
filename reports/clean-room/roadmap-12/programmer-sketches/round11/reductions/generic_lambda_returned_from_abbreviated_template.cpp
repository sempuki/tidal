// A generic lambda returned from a function template whose parameter is
// auto, then called. From views.cpp.
auto scale(auto k) {
  return [k](auto r) { return r * k; };
}
int main() { return scale(2)(3) == 6 ? 0 : 1; }
