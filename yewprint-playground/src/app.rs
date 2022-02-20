use crate::forms::*;
use crate::nav_menu::*;
use crate::page::*;
use yew::prelude::*;
use yew_router::router::Router;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="app-root bp3-dark">
                <div class="app-nav">
                    <div class="app-nav-header">
                        <a class="app-logo" href="/">
                            {crate::include_raw_html!("logo.svg")}
                        </a>
                        <div>
                            {"Yewprint-playground"}
                        </div>
                        <a
                            href="https://github.com/Yozhgoor/yewprint-playground"
                            target="_blank"
                        >
                            <small>{"View on GitHub"}</small>
                        </a>
                    </div>
                    <div>
                        <NavMenu />
                    </div>
                </div>
                <main class="app-content" role="main">
                    <div>
                        <Router<AppMenu, ()>
                            render=Router::render(|switch: AppMenu| {
                                match switch {
                                    AppMenu::Page | AppMenu::Home =>
                                        html! (<Page />),
                                    AppMenu::Forms => html! (<Forms />),
                                }
                            })
                        />
                    </div>
                </main>
            </div>
        }
    }
}
