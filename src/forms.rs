use yew::prelude::*;
use yewprint::{InputGroup, Text, H1, H2, H3};

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
                    <div class="forms-section">
                        <div class="forms-section-title">
                            <H2>{"User data"}</H2>
                        </div>
                        <div class="forms-section-content">
                            <div class="forms-name">
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"First Name"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: Tom"}
                                        />
                                    </div>
                                </div>
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Middle Name"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: G."}
                                        />
                                    </div>
                                </div>
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Last Name"}</Text>
                                    </div>
                                    <div class="forms-input">
                                        <InputGroup
                                            placeholder={"ex: Sawyer"}
                                        />
                                    </div>
                                </div>
                            </div>
                            <div class="forms-birth">
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Date of Birth"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: 01/01/1832"}
                                        />
                                    </div>
                                </div>
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Place of Birth"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: St. Petersburg, Missouri"}
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="forms-section-content">
                            <div class="forms-content-title">
                                <H3>{"Adress"}</H3>
                            </div>
                            <div class="forms-adress">
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Street"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: Main Street"}
                                        />
                                    </div>
                                </div>
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Number"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: 120"}
                                        />
                                    </div>
                                </div>
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Box"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: 1"}
                                        />
                                    </div>
                                </div>
                            </div>
                            <div class="forms-location">
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Zip Code"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: 63401"}
                                        />
                                    </div>
                                </div>
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"City"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"St. Petersburgh"}
                                        />
                                    </div>
                                </div>
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"State"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: Missouri"}
                                        />
                                    </div>
                                </div>
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Country"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: USA"}
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="forms-section-content">
                            <div class="forms-content-title">
                                <H3>{"Contact"}</H3>
                            </div>
                            <div class="forms-contact">
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Phone Number"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: +1 573-221-9010"}
                                        />
                                    </div>
                                </div>
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Email Adress"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: name@example.com"}
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class ="forms-footer">
                </div>
            </div>
        }
    }
}
