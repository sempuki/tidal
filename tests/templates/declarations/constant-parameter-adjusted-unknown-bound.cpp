// tidal-rules: TPL-8
// tidal-expect: accept
// tidal-note: An array of unknown bound becomes const char*, and a function type with a parameter becomes a pointer to function.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
constexpr char text[] = "hi";
int twice(int v) { return v * 2; }
template <const char S[]> struct Text { using type = decltype(S); };
template <int F(int)> struct Call { using type = decltype(F); };
static_assert(same<Text<text>::type, const char*>);
static_assert(same<Call<twice>::type, int (*)(int)>);
