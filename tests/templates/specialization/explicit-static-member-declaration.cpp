// tidal-rules: TSP-28 LNK-7
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .Box<char>::count
// tidal-error-gcc: undefined reference to .Box<char>::count
// tidal-note: The explicit specialization has no initializer, so it declares Box<char>::count and does not define it.
template <class T> struct Box { static int count; };
template <> int Box<char>::count;
int main() { return Box<char>::count; }
