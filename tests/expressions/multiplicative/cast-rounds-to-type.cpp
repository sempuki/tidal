// tidal-rules: MUL-17
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
volatile float a = 16777216.0f;
int main() { float r = static_cast<float>(a + 1.0f); return r == 16777216.0f ? 0 : 1; }
