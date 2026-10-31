// tidal-rules: ATTR-6
// tidal-expect: accept
// tidal-flags: -Werror -Wunused
struct Closer { ~Closer(); };
struct File : Closer { int handle; };
void f() {
  File file;
}
