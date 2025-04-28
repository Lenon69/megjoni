use leptos::prelude::*;

#[component]
pub fn TermsAndConditionsPage() -> impl IntoView {
    view! {
         // Użyto prostych klas Tailwind/podobnie nazwanych dla struktury - dostosuj do swojego CSS
        <section class="max-w-3xl mx-auto p-4 text-gray-800 dark:text-gray-200">
            <h1 class="text-3xl font-bold mb-6">"Regulamin sklepu"</h1>
            <p class="text-sm text-gray-500 mb-8">"Data ostatniej aktualizacji: 23 kwietnia 2025 r."</p>

            // Div otacza sekcje i akapity z treścią
            <div class="prose max-w-none">
                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"1. Postanowienia ogólne"</h2>
                    <p>
                        "Niniejszy regulamin określa zasady korzystania ze sklepu internetowego prowadzonego pod adresem www.megjoni.pl.
                        Sklep prowadzony jest przez [pełna nazwa firmy, adres, NIP, REGON]." // TODO: Uzupełnij dane firmy
                    </p>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"2. Składanie zamówień"</h2>
                    <ul class="list-disc list-inside">
                        <li>"Zamówienia można składać 24 godziny na dobę, 7 dni w tygodniu"</li>
                        <li>"Złożenie zamówienia oznacza akceptację niniejszego regulaminu"</li>
                        <li>"Po złożeniu zamówienia klient otrzymuje e-mail z potwierdzeniem przyjęcia zamówienia"</li>
                    </ul>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"3. Ceny i płatności"</h2>
                    <ul class="list-disc list-inside">
                        <li>"Wszystkie ceny podane w sklepie są cenami brutto i zawierają podatek VAT"</li>
                        <li>"Akceptowane formy płatności: przelew bankowy, szybkie płatności online, BLIK"</li> // TODO: Wymień dostępne formy płatności
                        <li>"Zamówienie jest realizowane po zaksięgowaniu płatności"</li>
                    </ul>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"4. Dostawa"</h2>
                    <p>"Informacje o kosztach i czasie dostawy znajdują się w zakładce Wysyłka i zwroty."</p>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"5. Zwroty i reklamacje"</h2>
                    <p>"Klient ma prawo do zwrotu towaru w ciągu 14 dni bez podania przyczyny. Szczegóły znajdują się w zakładce Wysyłka i zwroty."</p>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"6. Dane osobowe"</h2>
                    <p>"Szczegóły dotyczące przetwarzania danych osobowych znajdują się w Polityce Prywatności."</p>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"7. Postanowienia końcowe"</h2>
                    <ul class="list-disc list-inside">
                        <li>"Sklep zastrzega sobie prawo do zmiany regulaminu"</li>
                        <li>"W sprawach nieuregulowanych mają zastosowanie przepisy prawa polskiego"</li>
                        <li>"Spory będą rozstrzygane przez właściwy sąd powszechny"</li>
                    </ul>
                </section>
            </div> // Zamknięcie div.prose
        </section> // Zamknięcie zewnętrznej sekcji
    }
}
