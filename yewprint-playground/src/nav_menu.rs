use yew::prelude::*;
use yew_router::{
    agent::{RouteAgentDispatcher, RouteRequest},
    Switch,
};
use yewprint::{Menu, MenuItem};

pub struct NavMenu {
    link: ComponentLink<Self>,
    route_dispatcher: RouteAgentDispatcher,
}

pub enum Msg {
    GoToMenu(AppMenu),
}

impl Component for NavMenu {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavMenu {
            link,
            route_dispatcher: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GoToMenu(app_menu) => {
                self.route_dispatcher
                    .send(RouteRequest::ChangeRoute(app_menu.into()));
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Menu>
                    <MenuItem
                        text={html!("Page")}
                        href="#page"
                        onclick=self.link
                            .callback(|_| Msg::GoToMenu(AppMenu::Page))
                    />
                    <MenuItem
                        text={html!("Forms")}
                        href="#forms"
                        onclick=self.link
                            .callback(|_| Msg::GoToMenu(AppMenu::Forms))
                    />
                </Menu>
            </div>
        }
    }
}

#[derive(Debug, Copy, Clone, Switch)]
pub enum AppMenu {
    #[to = "/#page"]
    Page,
    #[to = "/#forms"]
    Forms,
    #[to = "/"]
    Home,
}
