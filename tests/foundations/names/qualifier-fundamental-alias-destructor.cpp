// tidal-rules: NAM-16
// tidal-expect: accept
// tidal-note: In a destructor name, the qualifier can name a scalar type through an alias.
using number = int;
void end(number* p) { p->number::~number(); }
