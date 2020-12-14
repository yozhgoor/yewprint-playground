use yew::prelude::*;
use yewprint::{Menu, MenuItem};

pub struct NavMenu {}

impl Component for NavMenu {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NavMenu {}
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
                <Menu>
                    <MenuItem />
                </Menu>
            </div>
        }
    }
}
