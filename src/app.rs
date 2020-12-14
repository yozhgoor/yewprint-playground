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
            <div>
                <div>
                    <div>
                        <a href="/">
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
                <main>
                    <div>
                        <Router<AppMenu, ()>
                            render=Router::render(|switch: AppMenu| {
                                match switch {
                                    AppMenu::Page | AppMenu::Home =>
                                    html! (<Page />),
                                }
                            })
                        />
                    </div>
                </main>
            </div>
        }
    }
}
