// tidal-rules: SINIT-3
// tidal-expect: run
// tidal-note: Holder<int>::v is a specialization of a template, so its place in the order is unspecified. first and third keep their order. Clang at -O0 gives 213, and GCC and Clang 22 at -O2 give 132.
int trace = 0;
int mark(int d) { trace = trace * 10 + d; return d; }
int first = mark(1);
template <class T> struct Holder { static int v; };
template <class T> int Holder<T>::v = mark(2);
int third = mark(3);
int main() {
  bool ordered = trace == 123 || trace == 213 || trace == 132;
  return Holder<int>::v == 2 && ordered ? 0 : 1;
}
