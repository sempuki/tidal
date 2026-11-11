using System;
using System.Collections.Generic;

// A lazy pipeline over a sequence: filter, map, take, and fold, each a
// stage that pulls from the one before it. The source counts how many
// values it produced, so the program can check that the pipeline is lazy.
static class Pipeline
{
    static int produced = 0;

    static IEnumerable<int> Naturals()
    {
        for (int i = 1; ; i++) { produced++; yield return i; }
    }

    static IEnumerable<T> Where<T>(IEnumerable<T> src, Func<T, bool> keep)
    {
        foreach (var x in src) if (keep(x)) yield return x;
    }

    static IEnumerable<U> Select<T, U>(IEnumerable<T> src, Func<T, U> f)
    {
        foreach (var x in src) yield return f(x);
    }

    static IEnumerable<T> Take<T>(IEnumerable<T> src, int n)
    {
        if (n <= 0) yield break;
        foreach (var x in src) { yield return x; if (--n == 0) yield break; }
    }

    static A Fold<T, A>(IEnumerable<T> src, A seed, Func<A, T, A> step)
    {
        var acc = seed;
        foreach (var x in src) acc = step(acc, x);
        return acc;
    }

    static int Main()
    {
        int failures = 0;
        var squaresOfOdd = Select(Where(Naturals(), x => x % 2 == 1), x => (long)x * x);
        long sum = Fold(Take(squaresOfOdd, 5), 0L, (a, x) => a + x);
        if (sum != 1 + 9 + 25 + 49 + 81) failures++;
        if (produced != 9) failures++;
        var words = new List<string> { "tide", "ebb", "flow", "surge", "neap" };
        int letters = Fold(Where(words, w => w.Length == 4), 0, (a, w) => a + w.Length);
        if (letters != 12) failures++;
        return failures;
    }
}
