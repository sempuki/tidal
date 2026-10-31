static const int factor = 3;
inline int scale(int v) { return v * factor; }
int scale_elsewhere(int v) { return scale(v); }
