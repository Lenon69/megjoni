use leptos::prelude::*;

#[component]
pub fn MenPage() -> impl IntoView {
    view! {
        <section>
            <h2>"Kategoria: Męska"</h2>
            <p>"Przeglądaj naszą ofertę męskiej odzieży używanej. Znajdź koszule, spodnie, marynarki i inne elementy garderoby w świetnych cenach."</p>

             // Placeholder dla siatki produktów męskich
            // TODO: W przyszłości ten grid powinien ładować dane produktów dynamicznie z API/DB
            <div class="product-grid">
                 <article class="product-item">
                     <a href="/product/meskie-001">
                         <figure>
                             <img src="/black-tshirt.jpg" alt="Czarny T-Shirt Męski" width="300" height="400" />
                         </figure>
                         <h3>"Czarny T-Shirt Męski"</h3>
                         <p class="product-price">"39.50 PLN"</p>
                     </a>
                 </article>
                 <article class="product-item">
                    <a href="/product/meskie-002">
                        <figure>
                            <img src="/niebieska-bluza.jpg" alt="Przykładowy produkt męski" width="300" height="400" />
                        </figure>
                        <h3>"Niebieska Bluza"</h3>
                        <p class="product-price">"85.00 PLN"</p>
                    </a>
                </article>
                // Dodaj więcej placeholderów produktów męskich
            </div>
        </section>
    }
}
