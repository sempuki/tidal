template <class It> constexpr It first(It a) { return a; }
struct S {
  int items[4]{1, 2, 3, 4};
  constexpr int get() const { return *first(items); }
};
static_assert(S{}.get() == 1);
int main() { return 0; }
