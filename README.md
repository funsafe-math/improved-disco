# Metoda Taylora dla równań różniczkowych
## Wstęp teoretyczny
Metoda Taylora pozwala nam na otrzymanie z dużą dokładnością przybliżonych wartości dla równania różniczkowego z zagadnieniem początkowym.

Jeżeli funkcja należy do klasy $C^{\infty}$ w otoczeniu punktu $x$, możemy ją przedstawić jako szereg Taylora:
$$
y(x+h) = y(x) + y'(x) \cdot h + \frac{y''(x)}{2!}\cdot h^2 + \frac{y'''(x)}{3!}\cdot h^3 + \dots
$$

Który można również zapisać w postaci z resztą Lagrange'a:
$$
y(x+h) = y(x) + y'(x) \cdot h + \frac{y''(x)}{2!}\cdot h^2 + \dots + \frac{y^{(n)}(x+\theta h)}{n!}\cdot h^n\quad \text{gdzie } \theta \in(0,1)
$$

Metodę pozwalającą na uzyskanie przybliżonej wartości funkcji w $x + h$,
znając wartość $f(x)$  poprzez odrzucenie wszystkich pochodnych od $(n + 1)$ stopnia wzwyż nazywamy _metodą Taylora $n$-tego rzędu_.

Znając wartość dokładną lub przybliżoną rozwiązania w punkcie $x_0$ możemy obliczyć wartość w 
$x_0+h$, za jej pomocą $x_0+2h$, a za jej pomocą $x_0+3h$ itd. ad infinitum.

Na błąd lokalny metody $n$-tego rzędu składa się odrzucona reszta, która wynosi 
$$
Rn=\frac{f^{(n + 1)}(x + \theta h)}{(n+1)!}\cdot h(n+1)\quad   \theta\in (0,1)
$$
Błędy lokalne uzyskiwane przy każdym kroku kumulują się dając w efekcie błąd globalny.

## Wymagania sprzętowe
System operacyjny
- Linux 5.7+
- MacOS
- Windows 10

Sprzęt
- RAM 4GB
- x86_64 CPU
- 100MB wolnej przestrzeni dyskowej

## Opis programu
Program pozwala na wizualizację rozwiązań szczególnych równań różniczkowych na
pewnym przedziale. Użytkownik ma możliwość wprowadzenia wartości dla zagadnienia
początkowego oraz pożądanej docelowej wartości błędu $\varepsilon$, z zakresu od $0,01$ do
$10,0$.

Na tej podstawie program wyświetla wykres przybliżonych wartości funkcji
obliczonych przy pomocy metody Taylora oraz wykres dokładnych wartości
uzyskanych z rozwiązania danego równania różniczkowego uzyskanego analitycznie.

Wykres jest uzyskiwany poprzez:
- założenie początkowej ilości przedziałów
- stworzenie dwóch wektorów wartości — wyprowadzonej analitycznie oraz przybliżonej
- liczona różnica między wartością dokładną a przybliżoną używając wzoru
$$
\varepsilon = \sqrt{\sum_i^n (x_i - \bar{x}_i)^2}
$$
- jeśli różnica jest mniejsza od zadanej dokładności — oba wyniki są prezentowane na wykresie
- w przeciwny wypadku, proces jest powtarzany dla zdwojonej liczby przedziałów