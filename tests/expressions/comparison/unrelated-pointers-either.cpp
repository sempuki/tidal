// tidal-rules: CMP-8
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
int a, b;
int main() { bool lt = &a < &b, gt = &a > &b, le = &a <= &b, ge = &a >= &b; (void)lt; (void)gt; (void)le; (void)ge; return 0; }
