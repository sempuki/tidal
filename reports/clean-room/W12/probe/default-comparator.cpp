template <class T, class Less = decltype([](const T& a, const T& b) { return a < b; })>
constexpr void sort(T* first, T* last, Less less = {}) {
  for (T* i = first; i != last; ++i)
    for (T* j = i; j != first && less(*j, *(j - 1)); --j) {
      T t = *j; *j = *(j - 1); *(j - 1) = t;
    }
}
constexpr bool f() { int a[] = {3, 1, 2}; sort(a, a + 3); return a[0] == 1 && a[2] == 3; }
static_assert(f());
int main() { return f() ? 0 : 1; }
