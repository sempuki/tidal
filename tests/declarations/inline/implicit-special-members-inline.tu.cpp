struct S { int v = 1; };
int other() { S a; S b = a; return b.v; }
