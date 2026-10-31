// tidal-rules: NAM-33
// tidal-expect: accept
struct point { int x = 1; int copy() const { point other = *this; return other.x; } };
int point = 0;
int read() { struct point p; return p.copy() + point; }
