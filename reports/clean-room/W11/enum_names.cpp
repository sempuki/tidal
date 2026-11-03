// An enum-to-name table, checked at compile time.
enum class Color : unsigned char { Red, Green, Blue, Yellow, Count };

constexpr const char* color_names[] = { "red", "green", "blue", "yellow" };

constexpr int count = static_cast<int>(Color::Count);
static_assert(sizeof color_names / sizeof color_names[0] == count,
              "one name per enumerator");

constexpr const char* name(Color c) {
  return color_names[static_cast<unsigned char>(c)];
}

constexpr bool same(const char* a, const char* b) {
  while (*a != '\0' && *a == *b) { ++a; ++b; }
  return *a == *b;
}

constexpr bool names_unique_and_nonempty() {
  for (int i = 0; i < count; ++i) {
    if (color_names[i][0] == '\0') return false;
    for (int j = i + 1; j < count; ++j)
      if (same(color_names[i], color_names[j])) return false;
  }
  return true;
}
static_assert(names_unique_and_nonempty());

// A switch gives a second mapping; check that both agree.
constexpr const char* name_by_switch(Color c) {
  using enum Color;
  switch (c) {
    case Red: return "red";
    case Green: return "green";
    case Blue: return "blue";
    case Yellow: return "yellow";
    case Count: break;
  }
  return "";
}

constexpr bool tables_agree() {
  for (int i = 0; i < count; ++i) {
    Color c = static_cast<Color>(i);
    if (!same(name(c), name_by_switch(c))) return false;
  }
  return true;
}
static_assert(tables_agree());
static_assert(same(name(Color::Blue), "blue"));

int main() {
  volatile int i = 3;
  return same(name(static_cast<Color>(i)), "yellow") ? 0 : 1;
}
