// A tagged union that owns its payload, written from the Tidal specification.
// Rules used: UNI-P2, UNI-3, UNI-4, UNI-7, UNI-11, UNI-14, UNI-6,
// LSUP-4, LSUP-7, STG-5, NEW-10, NEW-20, DTOR-6, CPY-2, CPY-3, CPY-9.

namespace std {
using size_t = decltype(sizeof 0);
}
// The non-allocating placement form (LSUP-7).
inline void* operator new(std::size_t, void* at) noexcept { return at; }
inline void operator delete(void*, void*) noexcept {}

int texts_alive = 0;

// A string that owns a heap array of chars.
class Text {
  char* chars;
  int length;

  static int count(const char* s) {
    int n = 0;
    while (s[n] != '\0') ++n;
    return n;
  }
  void copy_from(const char* s, int n) {
    chars = new char[n + 1];
    for (int i = 0; i < n; ++i) chars[i] = s[i];
    chars[n] = '\0';
    length = n;
  }

public:
  explicit Text(const char* s) { copy_from(s, count(s)); ++texts_alive; }
  Text(const Text& o) { copy_from(o.chars, o.length); ++texts_alive; }
  Text(Text&& o) noexcept : chars(o.chars), length(o.length) {
    o.chars = nullptr;
    o.length = 0;
    ++texts_alive;
  }
  Text& operator=(const Text& o) {
    if (this != &o) {
      delete[] chars;
      copy_from(o.chars, o.length);
    }
    return *this;
  }
  Text& operator=(Text&& o) noexcept {
    if (this != &o) {
      delete[] chars;
      chars = o.chars;
      length = o.length;
      o.chars = nullptr;
      o.length = 0;
    }
    return *this;
  }
  ~Text() { delete[] chars; --texts_alive; }

  int size() const { return length; }
  char at(int i) const { return chars[i]; }
};

class Value {
public:
  enum class Kind : unsigned char { none, number, text };

private:
  Kind kind;
  union {            // anonymous: number and text are variant members of Value
    long number;
    Text text;       // Text has non-trivial functions, so UNI-3 deletes
  };                 // Value's implicit ones, and Value writes its own

  void destroy() {
    if (kind == Kind::text) text.~Text();   // the union has no active member now
    kind = Kind::none;
  }
  void build_from(const Value& o) {
    switch (o.kind) {
      case Kind::none: break;
      case Kind::number: number = o.number; break;
      case Kind::text: new (&text) Text(o.text); break;   // text becomes active
    }
    kind = o.kind;
  }
  void build_from(Value&& o) {
    switch (o.kind) {
      case Kind::none: break;
      case Kind::number: number = o.number; break;
      case Kind::text: new (&text) Text(static_cast<Text&&>(o.text)); break;
    }
    kind = o.kind;
  }

public:
  Value() : kind(Kind::none) {}
  explicit Value(long n) : kind(Kind::number), number(n) {}
  explicit Value(const char* s) : kind(Kind::text), text(s) {}

  Value(const Value& o) : kind(Kind::none) { build_from(o); }
  Value(Value&& o) noexcept : kind(Kind::none) { build_from(static_cast<Value&&>(o)); }

  Value& operator=(const Value& o) {
    if (this != &o) {
      // Copy first, so a throw from the copy leaves *this unchanged.
      Value tmp(o);
      destroy();
      build_from(static_cast<Value&&>(tmp));
    }
    return *this;
  }
  Value& operator=(Value&& o) noexcept {
    if (this != &o) {
      destroy();
      build_from(static_cast<Value&&>(o));
    }
    return *this;
  }

  ~Value() { destroy(); }

  Kind which() const { return kind; }
  long as_number() const { return number; }
  const Text& as_text() const { return text; }
};

int main() {
  {
    Value a(42L);
    Value b("hello");
    Value c;
    if (a.which() != Value::Kind::number || a.as_number() != 42) return 1;
    if (b.which() != Value::Kind::text || b.as_text().size() != 5) return 2;
    if (c.which() != Value::Kind::none) return 3;
    if (texts_alive != 1) return 4;

    Value d = b;                    // copy: two Texts
    if (texts_alive != 2 || d.as_text().at(1) != 'e') return 5;

    Value e = static_cast<Value&&>(d);   // move: d's Text is now empty
    if (texts_alive != 3) return 6;
    if (e.as_text().size() != 5 || d.as_text().size() != 0) return 7;

    a = b;                          // number to text
    if (a.which() != Value::Kind::text || texts_alive != 4) return 8;
    b = Value(7L);                  // text to number: b's Text ends
    if (b.which() != Value::Kind::number || b.as_number() != 7) return 9;
    if (texts_alive != 3) return 10;

    Value& same = a;
    a = same;                       // self-assignment keeps the Text
    if (a.as_text().at(4) != 'o') return 11;

    c = static_cast<Value&&>(e);    // none to text, by move
    if (c.as_text().at(0) != 'h') return 12;
    if (texts_alive != 4) return 13;  // a, c, d (empty), e (empty)
  }
  if (texts_alive != 0) return 14;
  return 0;
}
