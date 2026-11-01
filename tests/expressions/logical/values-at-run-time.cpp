// tidal-rules: LOG-10
// tidal-expect: run
volatile bool t = true, f = false;
int main() {
  bool ok = (t && t) && !(t && f) && !(f && t) && !(f && f) &&
            (t || t) && (t || f) && (f || t) && !(f || f) && !f && !!t;
  return ok ? 0 : 1;
}
