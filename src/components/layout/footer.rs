use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="footer-links">
                <ul>
                    <li><a href="/kontakt">"Kontakt"</a></li>
                    <li><a href="/dostawa-i-zwroty">"Wysyłka i zwroty"</a></li>
                    <li><a href="/polityka-prywatnosci">"Polityka Prywatności"</a></li>
                    <li><a href="/regulamin">"Regulamin sklepu"</a></li>
                </ul>
            </div>

            <div class="social-media">
                <p>"Znajdź nas w social mediach:"</p>
                <a href="https://www.facebook.com/MegJoni"
                   target="_blank"
                   rel="noopener noreferrer"
                   aria-label="Facebook">
                    <img src="/facebook.svg" width="48" height="48"/>
                </a>
                <a href="https://www.instagram.com/Meg.joni"
                   target="_blank"
                   rel="noopener noreferrer"
                   aria-label="Instagram">
                    <img src="/instagram.svg" width="48" height="48"/>
                </a>
            </div>

            <div class="copyright">
                // Rok można ustawić dynamicznie w przyszłości
                <p>"©"<span id="current-year">"2025"</span>" Meg Joni. Wszelkie prawa zastrzeżone."</p>
            </div>
        </footer>
    }
}
