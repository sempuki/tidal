struct R { int id; int score; };
template <class T, class M> int max_by(T* b, T* e, M T::* m) { int best = b->*m; for (; b != e; ++b) if (b->*m > best) best = b->*m; return best; }
int main() { R r[] = {{1, 5}, {2, 9}, {3, 7}}; return max_by(r, r + 3, &R::score) == 9 ? 0 : 1; }
