// Probe from one-line/class_type_template_parameter.cpp: with Name<6>
// in place of the deduced Name, does tidal-fe run?
template <unsigned N> struct Name { char s[N]; constexpr Name(const char (&a)[N]) { for (unsigned i = 0; i < N; ++i) s[i] = a[i]; } };
template <Name<6> N> constexpr unsigned len() { return sizeof(N.s) - 1; }
int main() { return static_cast<int>(len<"hello">()) - 5; }
