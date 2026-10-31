extern int base;
inline int derived = base + 1;
int other() { return derived; }
