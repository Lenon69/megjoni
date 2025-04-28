use leptos::prelude::*;

#[component]
pub fn ShippingReturnsPage() -> impl IntoView {
    view! {
        // Użyto prostych klas Tailwind/podobnie nazwanych dla struktury - dostosuj do swojego CSS
        <section class="max-w-3xl mx-auto p-4 text-gray-800 dark:text-gray-200">
            <h1 class="text-3xl font-bold mb-6">"Wysyłka i zwroty"</h1>
            <p class="text-sm text-gray-500 mb-8">"Data ostatniej aktualizacji: 23 kwietnia 2025 r."</p>

            <div class="prose max-w-none"> // Klasa 'prose' często używana do stylizacji tekstu, dostosuj lub usuń
                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"1. Koszt i czas wysyłki"</h2>
                    <ul class="list-disc list-inside">
                        <li>"Koszt dostawy na terenie Polski: 14,99 zł"</li>
                        <li>"Czas realizacji zamówienia: 1–3 dni robocze"</li>
                        <li>"Czas dostawy: 1–2 dni robocze od momentu nadania"</li>
                        <li>"Darmowa dostawa dla zamówień powyżej [np. 200 zł]"</li> // TODO: Ustaw próg darmowej dostawy
                    </ul>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"2. Formy dostawy"</h2>
                    <ul class="list-disc list-inside">
                        <li>"Kurier (np. InPost, DPD, DHL)"</li> // TODO: Wymień dostępne formy dostawy
                        <li>"Paczkomaty InPost"</li>
                    </ul>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"3. Zwroty i reklamacje"</h2>
                    <p>"Zgodnie z prawem konsumenta masz prawo do zwrotu towaru w ciągu 14 dni od jego otrzymania – bez podania przyczyny."</p>
                    <ul class="list-disc list-inside mt-2">
                        <li>"Produkt nie może nosić śladów użytkowania i musi być odesłany w oryginalnym stanie"</li>
                        <li>"Zwrotu dokonujesz na własny koszt"</li>
                        <li>"Zwrot środków nastąpi do 14 dni od otrzymania przesyłki"</li>
                    </ul>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"4. Jak dokonać zwrotu?"</h2>
                    <ol class="list-decimal list-inside">
                        <li>"Wypełnij formularz zwrotu (dostępny w zakładce Zwroty lub dołączony do przesyłki)"</li> // TODO: Udostępnij formularz
                        <li>"Zapakuj produkt i odeślij na adres:"<br/>"[Adres do zwrotu]"</li> // TODO: Podaj dokładny adres do zwrotów
                        <li>"Po otrzymaniu i sprawdzeniu przesyłki dokonamy zwrotu pieniędzy"</li>
                    </ol>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"5. Reklamacje"</h2>
                    <p>"Jeśli produkt jest uszkodzony lub niezgodny z opisem, skontaktuj się z nami pod adresem e-mail: kontakt@megjoni.pl. Do reklamacji dołącz zdjęcia oraz numer zamówienia."</p>
                </section>

                <section class="mb-6">
                    <h2 class="text-xl font-semibold mb-2">"6. Kontakt"</h2>
                    <p>
                        "W razie pytań dotyczących wysyłki lub zwrotów:"<br/>
                        "📧 E-mail: kontakt@megjoni.pl"<br/>
                        "📬 Adres: Siedziba Łódź" // TODO: Podaj dokładny adres kontaktowy
                    </p>
                </section>
            </div>
        </section>
    }
}
