use yew::prelude::*;
use yewprint::{Button, IconName, InputGroup};

pub struct Page {}

impl Component for Page {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Page {}
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
                <InputGroup
                    round=true
                    placeholder={"Search..."}
                    right_element=html! {
                        <Button
                            icon=IconName::Search
                            minimal=true
                        />
                    }
                />
            </div>
        }
    }
}
