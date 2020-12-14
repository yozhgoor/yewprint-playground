use yew::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
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
                </div>
                <main>
                </main>
            </div>
        }
    }
}
