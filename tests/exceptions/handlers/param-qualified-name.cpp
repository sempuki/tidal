// tidal-rules: TRY-5
// tidal-expect: reject
namespace n { extern int x; }
void g() { try { } catch (int n::x) { } }
