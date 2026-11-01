// tidal-rules: ATTR-19
// tidal-expect: accept
[[tidal::note]] int a;
int b [[tidal::note]];
int [[tidal::note]] c;
int * [[tidal::note]] p;
int & [[tidal::note]] r = a;
int list[3] [[tidal::note]];
struct [[tidal::note]] Base {};
union [[tidal::note]] Either { int i; float f; };
enum [[tidal::note]] Color { red [[tidal::note]] };
namespace [[tidal::note]] tools {}
using Count [[tidal::note]] = int;
void declared() [[tidal::note]];
void take([[tidal::note]] int x);
int make() {
  int* q = new int [[tidal::note]];
  delete q;
  [[tidal::note]] int local = 0;
  [[tidal::note]] done:
  [[tidal::note]] return local;
}
