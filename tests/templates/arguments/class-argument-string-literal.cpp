// tidal-rules: TARG-16
// tidal-expect: accept
// tidal-note: A string literal initializes a class argument through a constructor that takes a pointer to const char.
struct Text {
  char data[8] = {};
  constexpr Text(const char* s) {
    for (int i = 0; i < 7 && s[i]; ++i) data[i] = s[i];
  }
};
template <Text T> struct Label { static constexpr char first = T.data[0]; };
static_assert(Label<"hello">::first == 'h');
