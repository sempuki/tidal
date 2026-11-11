// Port of Pipeline.cs. Each stage is a class with a next() that pulls one
// value from the stage before it, as an IEnumerator does with MoveNext.
// Coroutines are out of scope in Tidal (KW-1), so no stage uses co_yield.
// Each stage holds the stage before it by value (OBJ-9: a reference to the
// temporary that Where(...) returns would dangle at the end of the line).

static int produced = 0;

// Gives the type of an expression without evaluating it (only in decltype).
template <class T> T&& fake();

// foreach support (RFOR-2): begin() and end() can return different types,
// and the loop uses prefix ++, unary *, and != against the end marker.
struct EndMark {};

template <class S> struct Cursor {
    S* stage;
    typename S::value_type current;
    bool has;
    explicit Cursor(S* s) : stage(s), current(), has(s->next(current)) {}
    Cursor& operator++() { has = stage->next(current); return *this; }
    typename S::value_type operator*() const { return current; }
    bool operator!=(EndMark) const { return has; }
};

template <class D> struct Enumerable {
    Cursor<D> begin() { return Cursor<D>(static_cast<D*>(this)); }
    EndMark end() { return EndMark{}; }
};

struct Naturals : Enumerable<Naturals> {
    using value_type = int;
    int i = 0;
    bool next(int& out) { ++i; ++produced; out = i; return true; }
};

template <class T> struct ArraySource : Enumerable<ArraySource<T>> {
    using value_type = T;
    const T* items;
    int count;
    int pos = 0;
    ArraySource(const T* p, int n) : items(p), count(n) {}
    bool next(T& out) {
        if (pos == count) return false;
        out = items[pos++];
        return true;
    }
};

template <class Src, class F> struct WhereStage : Enumerable<WhereStage<Src, F>> {
    using value_type = typename Src::value_type;
    Src src;
    F keep;
    WhereStage(Src s, F k) : src(s), keep(k) {}
    bool next(value_type& out) {
        value_type x{};
        while (src.next(x)) {
            if (keep(x)) { out = x; return true; }
        }
        return false;
    }
};

template <class Src, class F> struct SelectStage : Enumerable<SelectStage<Src, F>> {
    using value_type = decltype(fake<F&>()(fake<typename Src::value_type&>()));
    Src src;
    F f;
    SelectStage(Src s, F g) : src(s), f(g) {}
    bool next(value_type& out) {
        typename Src::value_type x{};
        if (!src.next(x)) return false;
        out = f(x);
        return true;
    }
};

template <class Src> struct TakeStage : Enumerable<TakeStage<Src>> {
    using value_type = typename Src::value_type;
    Src src;
    int n;
    TakeStage(Src s, int count) : src(s), n(count) {}
    bool next(value_type& out) {
        if (n <= 0) return false;
        if (!src.next(out)) return false;
        --n;
        return true;
    }
};

template <class Src, class F> WhereStage<Src, F> Where(Src src, F keep) {
    return WhereStage<Src, F>(src, keep);
}

template <class Src, class F> SelectStage<Src, F> Select(Src src, F f) {
    return SelectStage<Src, F>(src, f);
}

template <class Src> TakeStage<Src> Take(Src src, int n) {
    return TakeStage<Src>(src, n);
}

template <class Src, class A, class F> A Fold(Src src, A seed, F step) {
    A acc = seed;
    for (auto x : src) acc = step(acc, x);
    return acc;
}

static int length(const char* s) {
    int n = 0;
    while (s[n] != '\0') ++n;
    return n;
}

int main() {
    int failures = 0;
    auto squaresOfOdd = Select(Where(Naturals(), [](int x) { return x % 2 == 1; }),
                               [](int x) { return (long)x * x; });
    long sum = Fold(Take(squaresOfOdd, 5), 0L, [](long a, long x) { return a + x; });
    if (sum != 1 + 9 + 25 + 49 + 81) failures++;
    if (produced != 9) failures++;
    const char* words[] = { "tide", "ebb", "flow", "surge", "neap" };
    int letters = Fold(Where(ArraySource<const char*>(words, 5),
                             [](const char* w) { return length(w) == 4; }),
                       0, [](int a, const char* w) { return a + length(w); });
    if (letters != 12) failures++;
    return failures;
}
