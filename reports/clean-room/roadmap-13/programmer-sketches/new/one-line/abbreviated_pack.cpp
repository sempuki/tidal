// An abbreviated function template with a pack and a fold.
constexpr auto sum(auto... xs) { return (0 + ... + xs); }
static_assert(sum(1, 2, 3L) == 6);
int main() { return sum() + sum(4, -4); }
