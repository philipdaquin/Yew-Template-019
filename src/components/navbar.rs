use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{AppRoute};
pub struct Navbar;

impl Component for Navbar { 
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <header class="navbar">
                <ul>
                    <li>
                        <Link<AppRoute> to={AppRoute::Home}><a>{"Home"}</a></Link<AppRoute>>
                    </li>
                    <li>
                        <Link<AppRoute> to={AppRoute::About}><a>{"About"}</a></Link<AppRoute>>
                    </li>
                </ul>
            </header>
            </>
        }

    }

}