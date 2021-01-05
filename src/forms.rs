use yew::prelude::*;
use yewprint::H1;

pub struct Forms {}

impl Component for Forms {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Forms {}
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
                <div class="forms-header">
                    <div class="forms-title">
                        <H1>{"Forms"}</H1>
                    </div>
                </div>
                <div class="forms-main">
                </div>
                <div class ="forms-footer">
                </div>
            </div>
        }
    }
}
