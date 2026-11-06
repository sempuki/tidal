// tidal-rules: RFOR-2 ADD-6
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction literal_view_range_for.cpp. A class with begin and end members over part of a string literal, in a range for, in constant evaluation and at run time.
struct Span {
  const char* data;
  int size;
  constexpr const char* begin() const { return data; }
  constexpr const char* end() const { return data + size; }
};
constexpr int count(Span s, char c) {
  int n = 0;
  for (char x : s)
    n += x == c;
  return n;
}
static_assert(count(Span{"seashell", 8}, 's') == 2);
int main() { return count(Span{"seashell", 3}, 'e') == 1 ? 0 : 1; }
