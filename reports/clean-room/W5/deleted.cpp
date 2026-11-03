// Overloads with deleted conversions.

constexpr int twice(int v) { return 2 * v; }
int twice(bool) = delete;
int twice(char) = delete;
int twice(double) = delete;
int twice(bool);                      // DEL-2: stays deleted

template <class T> int exact(T) = delete;
constexpr int exact(int v) { return v + 1; }

struct Celsius {
  constexpr Celsius(double d) : value(d) {}
  Celsius(int) = delete;
  double value;
};
constexpr double warm(Celsius c) { return c.value + 1.0; }

template <class T> concept Twiceable = requires(T v) { twice(v); };
template <class T> concept Exact = requires(T v) { exact(v); };
template <class T> concept Warmable = requires(T v) { warm(v); };

static_assert(twice(3) == 6);
static_assert(twice(short{4}) == 8);   // promotion beats conversions
static_assert(Twiceable<int>);
static_assert(Twiceable<short>);
static_assert(!Twiceable<bool>);       // exact match: deleted
static_assert(!Twiceable<char>);
static_assert(!Twiceable<double>);
static_assert(!Twiceable<float>);      // float to double is a promotion
static_assert(!Twiceable<long>);       // several conversions tie

static_assert(exact(1) == 2);          // plain beats the template
static_assert(Exact<int>);
static_assert(!Exact<short>);          // template is an exact match
static_assert(!Exact<long>);

static_assert(warm(20.0) == 21.0);
static_assert(Warmable<double>);
static_assert(!Warmable<int>);         // deleted constructor chosen

int main(int argc, char**) {
  int n = argc;                        // 1 when run plainly
  short s = static_cast<short>(n);
  if (twice(n) != 2) return 1;
  if (twice(s) != 2) return 2;
  if (exact(n) != 2) return 3;
  if (warm(Celsius{static_cast<double>(n)}) != 2.0) return 4;
  return 0;
}
