// tidal-rules: CLS-31 CLS-34 ATTR-9 CLS-10 CLS-35
// tidal-expect: run
// tidal-note: x86-64. An empty member with [[no_unique_address]] takes no bytes, and two such members of one type get two addresses. A marked member of a class with tail padding lends the padding to the next member under the same choice as a base (layout-base-tail-padding.cpp).
struct Empty {};
struct Plain { int a; char b; };
struct Built { Built() {} int a; char b; };
struct Beside { int value; [[no_unique_address]] Empty tag; };
struct Two { [[no_unique_address]] Empty first; [[no_unique_address]] Empty second; int value; };
struct Only { [[no_unique_address]] Empty first; [[no_unique_address]] Empty second; };
struct PlainThen { [[no_unique_address]] Plain part; char c; };
struct BuiltThen { [[no_unique_address]] Built part; char c; };
static_assert(sizeof(Beside) == sizeof(int) && sizeof(Two) == sizeof(int) && sizeof(Only) == 2);
static_assert(sizeof(PlainThen) == 12 && sizeof(BuiltThen) == 8);
int main() {
  Two two;
  const char* base = reinterpret_cast<const char*>(&two);
  const char* first = reinterpret_cast<const char*>(&two.first);
  const char* second = reinterpret_cast<const char*>(&two.second);
  return first != second && first - base < 4 && second - base < 4 ? 0 : 1;
}
