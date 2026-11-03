// A consteval validator for identifier strings.
consteval bool is_identifier(const char* s) {
  auto letter = [](char c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
  };
  if (!letter(*s)) return false;
  for (++s; *s != '\0'; ++s)
    if (!letter(*s) && !(*s >= '0' && *s <= '9')) return false;
  return true;
}

// A name that can only be built from a valid identifier.
struct Identifier {
  const char* text;
  consteval Identifier(const char* s) : text(s) {
    if (!is_identifier(s)) throw "not an identifier";
  }
};

static_assert(is_identifier("hello_world2"));
static_assert(!is_identifier("2fast"));
static_assert(!is_identifier(""));
static_assert(!is_identifier("a-b"));

constexpr Identifier good = "counter";

constexpr int length(const char* s) {
  int n = 0;
  while (s[n] != '\0') ++n;
  return n;
}

int run_time_length(Identifier id) { return length(id.text); }

int main() {
  Identifier local = "x1";  // checked at translation time
  return run_time_length(good) == 7 && run_time_length(local) == 2 ? 0 : 1;
}
