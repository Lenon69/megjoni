use leptos::prelude::*;

#[component]
pub fn NewsPage() -> impl IntoView {
    view! {
        <section>
            <h2>"Nowości u Meg Joni"</h2>
            <p>"Zobacz nasze najnowsze dostawy! Świeże i unikalne ubrania dodane do sklepu."</p>

             // Placeholder dla siatki najnowszych produktów
            // TODO: W przyszłości ten grid powinien ładować dane produktów dynamicznie z API/DB (posortowane wg daty dodania)
            <div class="product-grid">
                 <article class="product-item">
                     <a href="/product/nowosc-001">
                         <figure>
                             <img src="/bluza-oversize.jpg" alt="Najnowszy produkt" width="300" height="400" />
                         </figure>
                         <h3>"Bluza Oversize"</h3>
                         <p class="product-price">"65.00 PLN"</p>
                     </a>
                 </article>
                // Dodaj więcej placeholderów nowości
            </div>
        </section>
    }
}
