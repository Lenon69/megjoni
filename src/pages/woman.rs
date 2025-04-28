use leptos::prelude::*;

#[component]
pub fn WomanPage() -> impl IntoView {
    view! {
        <section>
            <h2>"Kategoria: Damska"</h2>
            <p>"Odkryj naszą kolekcję odzieży damskiej z drugiej ręki. Eleganckie sukienki, wygodne spodnie, stylowe bluzki i wiele więcej!"</p>

            // Placeholder dla siatki produktów damskich
            // TODO: W przyszłości ten grid powinien ładować dane produktów dynamicznie z API/DB
            <div class="product-grid">
                 <article class="product-item">
                     <a href="/product/damskie-001">
                         <figure>
                             <img src="/czerwona-sukienka.jpg" alt="Przykładowy produkt damski" width="300" height="400" />
                         </figure>
                         <h3>"Czerwona sukienka"</h3>
                         <p class="product-price">"75.00 PLN"</p>
                     </a>
                 </article>
                 <article class="product-item">
                    <a href="/product/damskie-002">
                        <figure>
                            <img src="/elegancka-sukienka.jpg" alt="Przykładowy produkt damski" width="300" height="400" />
                        </figure>
                        <h3>"Elegancka sukienka"</h3>
                        <p class="product-price">"55.00 PLN"</p>
                    </a>
                </article>
                // Dodaj więcej placeholderów produktów damskich
            </div>
        </section>
    }
}
