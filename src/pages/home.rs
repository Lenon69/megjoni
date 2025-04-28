use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        // Usunęliśmy znacznik <main> z komponentów stron,
        // ponieważ <main> jest już w głównym komponencie App
        <section class="hero-section">
            <img src="/clothes1.jpg"
                alt="Odkryj unikalne perełki z drugiej ręki"
                class="hero-image" />
            <div class="hero-text">
                <h2>"Moda z Duszą - Znajdź Swoje Unikalne Perełki"</h2>
                <p>"Wysokiej jakości odzież używana, starannie wyselekcjonowana dla Ciebie."</p>
                <a href="/woman">
                    <button>"Przejdź do sklepu"</button>
                </a>
            </div>
        </section>

        <section class="featured-products">
            <h2>"Polecane produkty"</h2>
             // TODO: W przyszłości ten grid powinien ładować dane produktów dynamicznie
             // Można stworzyć osobny komponent ProductItem
             <div class="product-grid">
                 <article class="product-item">
                     <a href="/product/spodnie-vintage"> // Nowa ścieżka z dynamicznym ID
                         <figure>
                             <img src=r"/spodnie-vintage.jpg" alt="Opis produktu. Np. Czerwona sukienka" width="300" height="400" />
                         </figure>
                         <h3>"Spodnie Vintage"</h3>
                         <p class="product-price">"49.99 PLN"</p>
                     </a>
                 </article>
             </div>
             <div class="view-all-link">
                 <a href="/woman">"Zobacz wszystkie produkty"</a>
             </div>
        </section>
        <section class="about-promo">
            <h2>"Dlaczego Second Hand?"</h2>
            <p>
                "Moda z drugiej ręki to świadomy wybór - ekologiczny, ekonomiczny i
                niepowtarzalny. Daj ubraniom drugie życie!"
            </p>
            <a href="/about" class="btn">
                <button>"Dowiedz się więcej o nas"</button>
            </a>
        </section>
    }
}
