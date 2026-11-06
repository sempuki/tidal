using Fn = int (*)(int);
static int twice(int v) { return v * 2; }
Fn pick() { return twice; }
