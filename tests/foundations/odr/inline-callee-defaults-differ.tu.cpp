int pick(int v = 2);
inline int use() { return pick(); }
int use_elsewhere() { return use(); }
