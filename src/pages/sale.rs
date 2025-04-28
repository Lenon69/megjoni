use leptos::prelude::*;

#[component]
pub fn SalePage() -> impl IntoView {
    view! {
        <section>
            <h2>"Wyprzedaż"</h2>
            <p>"Super okazje czekają! Ostatnie sztuki w niższych cenach."</p>

            // Placeholder dla siatki produktów na wyprzedaży
            // TODO: W przyszłości ten grid powinien ładować dane produktów dynamicznie z API/DB (tylko produkty z przeceną)
            <div class="product-grid">
                 <article class="product-item">
                     <a href="/product/sale-001">
                         <figure>
                             <img src="/letnia-sukienka.jpg" alt="Produkt na wyprzedaży" width="300" height="400" />
                         </figure>
                         <h3>"Letnia Sukienka"</h3>
                         // Przykład ceny wyprzedażowej (można dodać więcej logiki i stylizacji)
                         <p class="product-price"><del style="color: var(--color-text-light);">"50.00 PLN"</del> "30.00 PLN"</p>
                     </a>
                 </article>
                // Dodaj więcej placeholderów produktów na wyprzedaży
            </div>
        </section>
    }
}
