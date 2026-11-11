struct Error { virtual ~Error() = default; virtual int code() const { return 1; } };
struct NotFound : Error { int code() const override { return 404; } };
int find(int k) { if (k < 0) throw NotFound{}; return k; }
int main() { try { find(-1); } catch (const Error& e) { return e.code() == 404 ? 0 : 1; } return 2; }
