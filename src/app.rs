use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::{
    components::{Footer, Header, Navbar},
    pages::{
        AboutPage, ContactPage, HomePage, MenPage, NewsPage, PrivacyPage, SalePage,
        ShippingReturnsPage, TermsAndConditionsPage, WomanPage,
    },
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="pl">
            <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />

                    <title>
                      Meg Joni - Odzież Używana Online | Najlepsze Second Hand Odkrycia
                    </title>
                    <meta
                      name="description"
                      content="Odkryj unikalną odzież używaną wysokiej jakości w naszym sklepie online. Znajdź stylowe perełki z drugiej ręki w świetnych cenach. Ekologiczne zakupy modowe."
                    />
                    <meta
                      name="keywords"
                      content="odzież używana, second hand online, sklep vintage, ciuchy z drugiej ręki, moda ekologiczna, ubrania używane, outlet, odzież damska używana, odzież męska używana"
                    />
                    <meta name="author" content="Meg Joni" />

                    <meta name="robots" content="index, follow" />
                    <meta name="theme-color" content="#ffffff" />

                    <meta
                      property="og:title"
                      content="Meg Joni - Odkryj Unikalną Odzież Używaną Online"
                    />
                    <meta
                      property="og:description"
                      content="Znajdź stylowe perełki z drugiej ręki i odśwież swoją garderobę w ekologiczny sposób. Wysoka jakość w świetnych cenach!"
                    />
                    <meta property="og:type" content="website" />
                    <meta property="og:url" content="https://www.megjoni.pl/" />
                    <meta property="og:site_name" content="Meg Joni" />
                    <meta property="og:locale" content="pl_PL" />

                    <link rel="canonical" href="https://www.megjoni.pl/" />

                    <link rel="stylesheet" href="/style.css" />
                <MetaTags/>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <link rel="stylesheet" href="/style.css" />
            </head>
            <body>
                <Router>
                    <Layout/>
                </Router>
            </body>
        </html>
    }
}

#[component]
fn Layout() -> impl IntoView {
    view! {
        <Header/>
        <Navbar/>

        <main>
            <Routes fallback=|| view! { <p>"Nie znaleziono strony"</p> }>
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=StaticSegment("o-nas") view=AboutPage/>
                <Route path=StaticSegment("kontakt") view=ContactPage/>
                <Route path=StaticSegment("nowosci") view=NewsPage/>
                <Route path=StaticSegment("polityka-prywatnosci") view=PrivacyPage/>
                <Route path=StaticSegment("dostawa-i-zwroty") view=ShippingReturnsPage/>
                <Route path=StaticSegment("regulamin") view=TermsAndConditionsPage/>
                <Route path=StaticSegment("mezczyzni") view=MenPage/>
                <Route path=StaticSegment("kobiety") view=WomanPage/>
                <Route path=StaticSegment("sale") view=SalePage/>
            </Routes>
        </main>

        <Footer/>
    }
}
