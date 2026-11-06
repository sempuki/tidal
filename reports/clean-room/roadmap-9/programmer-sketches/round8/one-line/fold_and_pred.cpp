template <class... T> constexpr bool all_pos(T... t) { return (... && (t > 0)); }
static_assert(all_pos(1, 2L, 3u) && !all_pos(1, -2));
int main() {}
