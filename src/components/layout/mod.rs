// Deklarujemy komponenty zawarte w tym katalogu
pub mod footer;
pub mod header;
pub mod navbar;

// Eksportujemy je, aby były dostępne w module components
pub use footer::*;
pub use header::*;
pub use navbar::*;
