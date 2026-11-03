// tidal-rules: NAM-40
// tidal-expect: run
// tidal-note: A using declaration brings both the class and the variable. The name alone then finds the variable.
namespace lib { struct handle { int v = 4; }; int handle = 3; }
using lib::handle;
int main() {
  struct handle h;
  return handle + h.v == 7 ? 0 : 1;
}
