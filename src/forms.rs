use yew::prelude::*;
use yewprint::{Button, ButtonGroup, HtmlSelect, InputGroup, Switch, Text, H1, H2, H3};

pub struct Forms {
    link: ComponentLink<Self>,
    ed_level: EdLevel,
}

impl Component for Forms {
    type Message = EdLevel;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Forms {
            link,
            ed_level: EdLevel::Elementary,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.ed_level = msg;
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
                    <div class="forms-horizontal-section">
                        <div class="first-horizontal-element">
                            <div class="forms-section-title">
                                <H2>{"Mode of Transportation"}</H2>
                            </div>
                            <div class="forms-section-content">
                                <div class="forms-transportation">
                                    <Switch
                                        label=html!("Foot")
                                    />
                                    <Switch
                                        label=html!("Bicycle")
                                    />
                                    <Switch
                                        label=html!("Car")
                                    />
                                    <Switch
                                        label=html!("Bus")
                                    />
                                    <Switch
                                        label=html!("Train")
                                    />
                                </div>
                            </div>
                        </div>
                        <div class="second-horizontal-element">
                            <div class="forms-section-title">
                                <H2>{"Sought Position"}</H2>
                            </div>
                            <div class="forms-section-content">
                                <div class="forms-position">
                                    <Switch
                                        label=html!("Worker")
                                    />
                                    <Switch
                                        label=html!("Driver")
                                    />
                                    <Switch
                                        label=html!("Employee")
                                    />
                                    <Switch
                                        label=html!("Secretary")
                                    />
                                    <Switch
                                        label=html!("Executive")
                                    />
                                </div>
                            </div>
                        </div>
                        <div class="third-horizontal-element">
                            <div class="forms-section-title">
                                <H2>{"Preferred Schedules"}</H2>
                            </div>
                            <div class="forms-section-content">
                                <div class="forms-schedules">
                                    <Switch
                                        label=html!{"Morning (7AM - 3PM)"}
                                    />
                                    <Switch
                                        label=html!{"Day (2PM - 10PM)"}
                                    />
                                    <Switch
                                        label=html!{"Night (9PM - 8AM)"}
                                    />
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="forms-section">
                        <div class="forms-section-title">
                            <H2>{"Education"}</H2>
                        </div>
                        <div class="forms-section-content">
                            <div class="forms-education">
                                <div class="forms-selection">
                                    <div class="forms-label">
                                        <Text>{"Education Level"}</Text>
                                    </div>
                                    <div class="forms-select">
                                        <HtmlSelect<EdLevel>
                                            options={vec![
                                                (EdLevel::Elementary, "Elementary School".to_string()),
                                                (EdLevel::Middle, "Middle School".to_string()),
                                                (EdLevel::High, "High School".to_string()),
                                                (EdLevel::University, "University/College".to_string()),
                                            ]}
                                            value=Some(self.ed_level)
                                            onchange=self.link.callback(|x| x)
                                            title=format!("Selected: {:?}", self.ed_level)
                                        />
                                    </div>
                                </div>
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Etablissement"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: Harvard University"}
                                        />
                                    </div>
                                </div>
                                <div class="forms-input">
                                    <div class="forms-label">
                                        <Text>{"Last Year"}</Text>
                                    </div>
                                    <div class="forms-field">
                                        <InputGroup
                                            placeholder={"ex: 2019-2020"}
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class ="forms-footer">
                    <div class="forms-button">
                        <ButtonGroup>
                            <Button>
                                {"Cancel"}
                            </Button>
                            <Button>
                                {"Save"}
                            </Button>
                        </ButtonGroup>
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum EdLevel {
    Elementary,
    Middle,
    High,
    University,
}
