// tidal-rules: TPL-6 TPL-7
// tidal-expect: run
// tidal-note: Roadmap round 12, programmer probe class_type_parameter_explicit_arguments.cpp. Tag<4> has a constexpr constructor from an array of char and one public member char[4], so Tag<4> is a structural type (TPL-6). The string literal "abc" converts to Tag<4> through the constructor in the template argument.
template <unsigned N> struct Tag {
  char text[N];
  constexpr Tag(const char (&from)[N]) : text{} {
    for (unsigned i = 0; i < N; ++i) text[i] = from[i];
  }
};
template <Tag<4> T> constexpr char last() { return T.text[2]; }
static_assert(last<"abc">() == 'c');
int main() { return last<"xyz">() == 'z' ? 0 : 1; }
