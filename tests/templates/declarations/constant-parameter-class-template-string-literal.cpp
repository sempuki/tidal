// tidal-rules: TPL-11 CTAD-1
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer reduction class_template_constant_parameter_from_string_literal.cpp and probes class_template_constant_parameter_member_read.cpp and class_template_constant_parameter_object.cpp (from keys_first_try.cpp). The type of the constant parameter of Tag is the class template Label with no argument list (TPL-11). A string literal argument deduces N from the length of the array, as for a variable of type Label initialized by the string literal (CTAD-1). The program reads static members of Tag for three arguments and declares an object of Tag<"id">.
using size = decltype(sizeof 0);
template <size N> struct Label {
  char text[N]{};
  constexpr Label(const char (&from)[N]) {
    for (size i = 0; i < N; ++i) text[i] = from[i];
  }
};
template <Label L> struct Tag {
  static constexpr char first = L.text[0];
  static constexpr size length = sizeof(L.text) - 1;
};
static_assert(Tag<"key">::first == 'k' && Tag<"key">::length == 3);
Tag<"id"> held;
int main() { return Tag<"z">::first == 'z' && Tag<"name">::length == 4 ? 0 : 1; }
