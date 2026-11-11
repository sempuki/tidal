// The noexcept operator on calls with and without the specifier.
void may();
void never() noexcept;
struct S { S() noexcept(false); ~S(); };
int main() {
  static_assert(!noexcept(may()) && noexcept(never()) && !noexcept(S()));
  return 0;
}
