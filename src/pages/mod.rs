// Deklarujemy wszystkie moduły dla stron
pub mod about;
pub mod contact;
pub mod home;
pub mod man;
pub mod news;
pub mod privacy;
pub mod sale;
pub mod shipping_returns;
pub mod terms_and_conditions;
pub mod woman;

// Eksportujemy wszystkie komponenty stron
pub use about::*;
pub use contact::*;
pub use home::*;
pub use man::*;
pub use news::*;
pub use privacy::*;
pub use sale::*;
pub use shipping_returns::*;
pub use terms_and_conditions::*;
pub use woman::*;
