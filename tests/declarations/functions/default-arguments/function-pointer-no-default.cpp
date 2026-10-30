// tidal-rules: DEFARG-5
// tidal-expect: reject
int f(int a = 1);
int (*p)(int) = f;
int x = p();
