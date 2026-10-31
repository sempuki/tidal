// tidal-rules: ODR-3 ODR-8
// tidal-expect: link-fail
// tidal-expect-gcc-16-o2: run
// tidal-expect-clang-22-o2: run
// tidal-error-clang: undefined reference to .Limits::top
// tidal-error-gcc: undefined reference to .Limits::top
// tidal-note: At -O2, GCC and Clang read the value through the reference at translation time and emit no reference to Limits::top, so the program links (ODR-8 needs no diagnostic).
struct Limits { static const int top = 5; };
int read(const int& value) { return value; }
int main() { return read(Limits::top) == 5 ? 0 : 1; }
