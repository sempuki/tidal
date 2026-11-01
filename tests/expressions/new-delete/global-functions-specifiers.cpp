// tidal-rules: NEW-19 UN-18
// tidal-expect: accept
void check(void* p, decltype(sizeof 0) n) {
  static_assert(!noexcept(::operator new(n)));
  static_assert(!noexcept(::operator new[](n)));
  static_assert(noexcept(::operator delete(p)));
  static_assert(noexcept(::operator delete[](p)));
}
int main() {}
