use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav>
            <ul>
                <li><a href="/">"Strona Główna"</a></li>
                <li><a href="/kobiety">"Damska"</a></li>
                <li><a href="/mezczyzni">"Męska"</a></li>
                <li><a href="/nowosci">"Nowości"</a></li>
                <li><a href="/wyprzedaz">"Wyprzedaż"</a></li>
                <li><a href="/o-nas">"O Nas"</a></li>
                <li><a href="/kontakt">"Kontakt"</a></li>
            </ul>
        </nav>
    }
}
