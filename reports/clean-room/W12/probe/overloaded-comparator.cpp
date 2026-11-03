template <class T, class Less> constexpr void sort(T*, T*, Less) {}
constexpr bool lt(int a, int b) { return a < b; }
constexpr bool lt(double a, double b) { return a < b; }
int main() { int a[2] = {2, 1}; sort(a, a + 2, lt); return 0; }
