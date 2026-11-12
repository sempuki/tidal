template <class It> constexpr It first(It a, It) { return a; }
constexpr int get() { const int items[4]{1, 2, 3, 4}; return *first(items, items + 4); }
static_assert(get() == 1);
int main() { return 0; }
