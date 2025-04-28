use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <div class="logo-title">
                <a href="/">
                    <img
                        src="/megjoni-big.png"
                        alt="Nazwa Sklepu Meg Joni / Logo"
                        height="100%" // Użycie 100% może wymagać odpowiedniego stylu kontenera
                    />
                </a>
            </div>

            <div class="search-bar">
                <form action="/search" method="get"> // Akcja formularza powinna wskazywać na endpoint wyszukiwania
                    <label for="search-input" class="visually-hidden">
                        "Szukaj produktów"
                    </label>
                    <input
                        type="text"
                        id="search-input"
                        name="query" // Nazwa parametru zapytania
                        placeholder="Szukaj..."
                    />
                    <button type="submit">"Szukaj"</button>
                </form>
            </div>

            <div class="user-cart-icons">
                <a href="/account" aria-label="Moje konto">
                    <img src="/my-account.svg" width="32" height="32"/>
                </a>
                <a href="/cart" aria-label="Mój koszyk">
                    <img src="/shopping-cart.svg" width="32" height="32"/>
                    // Liczba produktów w koszyku - w rzeczywistości powinna być stanem
                    <span class="cart-count">"0"</span>
                </a>
            </div>
        </header>
    }
}
