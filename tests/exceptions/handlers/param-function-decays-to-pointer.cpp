// tidal-rules: TRY-5
// tidal-expect: accept
int pick();
void g() { try { throw pick; } catch (int ()) { } }
