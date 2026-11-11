// CTAD-9, CTAD-27: a pack of bases takes every element.
template <class... F> struct Overload : F... { using F::operator()...; };
int main() {
  Overload pick{[](int) { return 1; }, [](double) { return 2; }, [](const char*) { return 3; }};
  return pick(1) + pick(1.0) + pick("x") == 6 ? 0 : 1;
}
