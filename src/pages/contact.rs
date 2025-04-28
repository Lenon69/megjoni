use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        // Możesz przenieść te inline style do pliku CSS
        <section class="contact-section" style="max-width: 600px; margin: var(--space-md) auto; background-color: var(--color-surface); padding: var(--space-md); border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);">
            <h2>"Skontaktuj się z Nami"</h2>
            <p>
                "Masz pytania dotyczące produktów, zamówień, czy współpracy? Chętnie pomożemy!
                Skontaktuj się z nami poprzez formularz poniżej lub bezpośrednio."
            </p>

            <div class="contact-info" style="margin-bottom: var(--space-md);">
                <p><strong>"Email: "</strong> <a href="mailto:kontakt@megjoni.pl">"kontakt@megjoni.pl"</a></p>
                // Dodaj opcjonalnie telefon, adres itp.
            </div>

            // To jest tylko placeholder, rzeczywista obsługa formularza wymaga backendu
            <form action="/submit-contact-form" method="post">
                <div style="margin-bottom: var(--space-sm);">
                    <label for="name" class="visually-hidden">"Twoje imię:"</label>
                    <input type="text" id="name" name="name" placeholder="Twoje imię" required style="width: 100%; padding: var(--space-xs); border: 1px solid var(--color-border); border-radius: 4px;" />
                </div>
                <div style="margin-bottom: var(--space-sm);">
                    <label for="email" class="visually-hidden">"Twój email:"</label>
                    <input type="email" id="email" name="email" placeholder="Twój email" required style="width: 100%; padding: var(--space-xs); border: 1px solid var(--color-border); border-radius: 4px;" />
                </div>
                 <div style="margin-bottom: var(--space-sm);">
                    <label for="subject" class="visually-hidden">"Temat:"</label>
                    <input type="text" id="subject" name="subject" placeholder="Temat wiadomości" style="width: 100%; padding: var(--space-xs); border: 1px solid var(--color-border); border-radius: 4px;" />
                </div>
                <div style="margin-bottom: var(--space-sm);">
                    <label for="message" class="visually-hidden">"Twoja wiadomość:"</label>
                    <textarea id="message" name="message" placeholder="Twoja wiadomość" rows="6" required style="width: 100%; padding: var(--space-xs); border: 1px solid var(--color-border); border-radius: 4px;"></textarea>
                </div>
                <button type="submit">"Wyślij wiadomość"</button>
            </form>
        </section>
    }
}
