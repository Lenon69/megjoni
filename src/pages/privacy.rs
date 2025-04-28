use leptos::prelude::*;

#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        // Użyto prostych klas Tailwind/podobnie nazwanych dla struktury - dostosuj do swojego CSS
        <section class="max-w-3xl mx-auto p-4 text-gray-800 dark:text-gray-200">
            <h1 class="text-3xl font-bold mb-6">"Polityka Prywatności"</h1>
            <p class="text-sm text-gray-500 mb-8">"Data ostatniej aktualizacji: 23 kwietnia 2025 r."</p>

            <div class="prose max-w-none"> // Klasa 'prose' często używana do stylizacji tekstu, dostosuj lub usuń
                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"1. Administrator danych osobowych"</h2>
                    <p>"Administratorem Twoich danych osobowych jest Magdalena Kluba, prowadząca działalność pod nazwą \"Meg Joni\". Możesz się z nami skontaktować pod adresem e-mail: kontakt@megjoni.pl."</p>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"2. Jakie dane zbieramy?"</h2>
                    <ul class="list-disc list-inside">
                        <li>"imię i nazwisko"</li>
                        <li>"adres dostawy"</li>
                        <li>"adres e-mail"</li>
                        <li>"numer telefonu (opcjonalnie)"</li>
                        <li>"dane do faktury (jeśli dotyczy)"</li>
                        <li>"adres IP oraz dane o aktywności na stronie (cookies – patrz pkt 6)"</li>
                    </ul>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"3. Cel i podstawa prawna przetwarzania danych"</h2>
                    <ul class="list-disc list-inside">
                        <li>"realizacja zamówień (art. 6 ust. 1 lit. b RODO)"</li>
                        <li>"prowadzenie konta użytkownika (jeśli dotyczy)"</li>
                        <li>"kontakt z klientem (art. 6 ust. 1 lit. f RODO)"</li>
                        <li>"cele księgowe (art. 6 ust. 1 lit. c RODO)"</li>
                        <li>"cele marketingowe za zgodą (art. 6 ust. 1 lit. a RODO)"</li>
                    </ul>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"4. Czas przechowywania danych"</h2>
                    <p>"Dane przechowujemy do czasu realizacji umowy i przez okres wymagany przepisami prawa. Dane wykorzystywane do celów marketingowych – do momentu cofnięcia zgody."</p>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"5. Udostępnianie danych"</h2>
                    <p>"Dane mogą być przekazywane firmom kurierskim, operatorom płatności, biuru księgowemu oraz firmie hostingowej – tylko w zakresie niezbędnym do świadczenia usług."</p>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"6. Pliki cookies"</h2>
                    <p>"Używamy cookies do działania strony, analizy ruchu (np. Google Analytics) i personalizacji treści. Możesz zmienić ich ustawienia w przeglądarce."</p>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"7. Twoje prawa"</h2>
                    <ul class="list-disc list-inside">
                        <li>"dostęp do danych"</li>
                        <li>"sprostowanie, usunięcie lub ograniczenie przetwarzania"</li>
                        <li>"przenoszenie danych"</li>
                        <li>"sprzeciw wobec przetwarzania"</li>
                        <li>"cofnięcie zgody"</li>
                        <li>"skarga do Prezesa UODO"</li>
                    </ul>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"8. Kontakt"</h2>
                    <p>
                        "W sprawach związanych z ochroną danych osobowych, skontaktuj się z nami:"<br/>
                        "📧 E-mail: kontakt@megjoni.pl"<br/>
                        "📬 Adres: Siedziba Łódź" // TODO: Podaj dokładny adres
                    </p>
                </section>
            </div>
        </section>
    }
}
