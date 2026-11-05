int& left();
int& right() { static int w = left() + 1; return w; }
