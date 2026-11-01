// tidal-rules: CE-29 CE-3
// tidal-expect: accept
// tidal-note: view is not constexpr, but its initializer is a constant expression, so view is usable (CE-29). The array that view names is constexpr, so a read of its element is valid (CE-3).
static constexpr int table[] = {1, 2};
auto&& view = table;
constexpr int second = view[1];
static_assert(second == 2);
