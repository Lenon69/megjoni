use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <section class="about-promo"> // Używamy istniejącej klasy CSS dla spójności
            <div class="container"> // Dodaj klasy kontenera, jeśli masz je w CSS
                <h2>"O Nas"</h2>
                <img src="/megjoni-big.png" id="megjoni-logo" alt="Meg Joni Logo"/> // Upewnij się, że ścieżka i alt są poprawne
            </div>
            <p>
                "Witaj w Meg Joni! Jesteśmy pasjonatami mody z drugiej ręki, wierzymy, że ubrania
                zasługują na drugie życie. Nasz sklep to miejsce, gdzie znajdziesz unikalne
                perełki vintage i starannie wyselekcjonowaną odzież używaną w doskonałym stanie."
            </p>
            <p>
                "Naszą misją jest promowanie zrównoważonej mody i pokazywanie, że można ubierać się
                stylowo, dbając jednocześnie o planetę. Każdy zakup w naszym sklepie to krok
                w stronę bardziej świadomego konsumpcjonizmu."
            </p>
            <p>
                "Dołącz do naszej społeczności miłośników second handu i odkryj swój niepowtarzalny styl!"
            </p>
             <a href="/woman">
                <button>"Zobacz nasze produkty"</button>
             </a>
        </section>
    }
}
