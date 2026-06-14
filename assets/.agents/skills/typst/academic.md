# Math Equations

For syntax fundamentals, see [basics.md](basics.md).

## Inline and Display Math

```typst
The formula $E = m c^2$ is inline.

$ integral_0^infinity e^(-x^2) dif x = sqrt(pi) / 2 $
```

## Aligned Equations

```typst
$ X_p^f [k] &= 1/T sum_(n=-oo)^oo integral_0^T x(t - n T) e^(-j (2pi k)/T t) d t \
             &= 1/T X^f ((2pi k)/T) $
```

## Common Math Patterns

| Pattern | Typst |
|---------|-------|
| Fraction | `a/b` or `frac(a, b)` |
| Square root | `sqrt(x)` |
| Subscript/superscript | `x_n^2` |
| Integral | `integral_(-oo)^oo f(t) d t` |
| Sum | `sum_(n=0)^(N-1) x[n]` |
| Limit | `lim_(n -> oo)` |
| Piecewise | `cases(1 & 0 <= n <= N-1, 0 & "אחרת")` |
| Matrix | `mat(a, b; c, d)` |
| Absolute value | `abs(x)` |
| Norm | `norm(x)` |
| Conjugate | `overline(X^f)` |
| Dot product | `a dot b` |
| Therefore | `therefore` |
| Approximately | `approx` |
| Cancel | `cancel(V_A)` |
| Underbrace | `underbrace(expr, "label")` |
| Text in math | `"word"` |

## Accents

```typst
$accent(x, hat)$    // x̂
$accent(x, macron)$ // x̄
$accent(x, arrow)$  // x⃗
$accent(x, tilde)$  // x̃
$accent(x, dot)$    // ẋ

// Shorthand helpers (define at top of file)
#let Hat(x) = $accent(#x, hat)$
#let Bar(x) = $accent(#x, macron)$
#let Vec(x) = $accent(#x, arrow)$
```

## Footnote Inside Math

```typst
$ E_"max" =^((#footnote()[נובע מנוסחת שטח משולש])) (2 V_(B I))/(d_n + d_p) $
```

## Left-align Math in RTL Pages

```typst
#align(left, block($rho_p = -q N_A = -1.6 dot 10^(-19) [C] dot 3.01 dot 10^(17) [cm^-3]$))
```

## Equation Numbering

```typst
#set math.equation(numbering: "(1)")

$ X^f (omega) = integral_(-oo)^oo x(t) e^(-j omega t) d t $ <eq:fourier>

As shown in @eq:fourier...
```

## Physics/Engineering Units in Math

```typst
$ d_n approx 2.24 dot 10^(-6) [cm] $
$ E_"max" approx 2.4 dot 10^5 [V/cm] $
$ V_(B I) = 0.9 [V] $
```
