// tidal-rules: STO-9
// tidal-expect: run-fail
// tidal-error: recursive_init_error
// tidal-note: The constructor of the static object calls the function that declares the object, so control reaches the declaration again during the initialization. The run-time library of Clang and GCC detects the second entry and ends the program.
struct Widget;
Widget& instance();
int built = 0;
struct Widget {
  int id;
  Widget() : id(++built) {
    if (built < 3) instance();
  }
};
Widget& instance() {
  static Widget w;
  return w;
}
int main() { return instance().id; }
