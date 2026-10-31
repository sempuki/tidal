// tidal-rules: REF-6
// tidal-expect: accept
constexpr int count = 1;
constexpr const int& alias{count};
static_assert(&alias == &count);

struct Base { int v; };
struct Derived : Base {};
constexpr Derived item{};
constexpr const Base& base{item};
static_assert(&base == &item);

struct Point { int x, y; };
constexpr Point origin{0, 0};
constexpr const Point& same{origin};
static_assert(&same == &origin);
