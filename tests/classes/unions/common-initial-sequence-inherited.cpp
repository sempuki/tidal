// tidal-rules: UNI-15 UNI-10 CLS-19
// tidal-expect: run
// tidal-note: Named declares no members, and its base declares them all, so Named is standard-layout and its members are those of Base. They match the members of Direct.
struct Base { int id; int size; };
struct Named : Base {};
struct Direct { int id; int size; };
union Entry { Named named; Direct direct; };
int main() {
  Entry e{.named = {{5, 6}}};
  return e.direct.id == 5 && e.direct.size == 6 ? 0 : 1;
}
