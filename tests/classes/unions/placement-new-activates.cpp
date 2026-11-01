// tidal-rules: UNI-14
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
inline void* operator new(decltype(sizeof 0), void* place) noexcept { return place; }
struct Name {
  const char* text;
  Name(const char* t) : text(t) {}
  ~Name() {}
};
union Holder {
  Name name;
  long raw;
  Holder() : raw(0) {}
  ~Holder() {}
};
int main() {
  Holder holder;
  Name* made = new (&holder.name) Name("tide");
  bool same = made == &holder.name;
  char first = holder.name.text[0];
  holder.name.~Name();
  return same && first == 't' ? 0 : 1;
}
