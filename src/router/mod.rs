pub mod about;
pub mod home;

use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {  
    #[at("/")]
    Home,
    
    #[at("/about")]
    About,
    
}
