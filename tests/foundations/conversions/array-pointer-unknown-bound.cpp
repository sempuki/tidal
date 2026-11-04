// tidal-rules: CONV-16
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
int a[3];
int (*p)[3] = &a;
int (*q)[] = p;
