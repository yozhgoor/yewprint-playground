# yewprint-playground

This repository is a [Yew](https://github.com/yewstack/yew) +
[yewprint](https://github.com/cecton/yewprint) playground using
[wasm-run](https://github.com/IMI-eRnD-Be/wasm-run).

## Development Server

```
cargo run -- serve
```

You can now go to http://localhost:3000

**Note:** when you make changes in the source code, you just have to reload the page
to see your changes.

## Production Build

```
cargo run -- build
```

## Playground

### Menu Component

If you want to configure the left component (with the logo and a menu for now) you can do this here:

[`src/nav_menu.rs`](https://github.com/Yozhgoor/yewprint-playground/blob/main/src/nav_menu.rs)

For example you can add a button after the menu like this:

Import the yewprint button
```rust
use yewprint::{Button, Menu, MenuItem};
```

Add the button
```rust
<div>
    <Menu>
        <MenuItem
            text={html!("Page")}
            href="#page"
            onclick=self.link
                .callback(|_| Msg::GoToMenu(AppMenu::Page))
        />
    </Menu>
    <Button fill=true>
        {"Ready?"}
    </Button>
</div>
```

### Page Component

If you want to configure the right component (with the "hello world") you can do this here:

[`src/page.rs`](https://github.com/Yozhgoor/yewprint-playground/blob/main/src/page.rs)

For example you can add a search field in the page like this:

Import the yewprint component needed
```rust
use yewprint::{Button, IconName, InputGroup};
```

Add the search field
```rust
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
```

### Styling with css

If you want to add some stylling you can use the
['static/app.css](https://github.com/Yozhgoor/yewprint-playground/blob/main/static/app.css)
file

For example, you can use some css rules on your newest elements like this:

Give a new class to your button
```rust
<div>
    <Menu>
        <MenuItem
            text={html!("Page")}
            href="#page"
            onclick=self.link
                .callback(|_| Msg::GoToMenu(AppMenu::Page))
        />
    </Menu>
    <Button
        class="nav-button"
        fill=true
    >
        {"Ready?"}
    </Button>
</div>
```

Give a new class to your search field
```rust
<div class="page-search">
    <InputGroup
        round=true
        placeholder={"Search..."}
        right_element=html! {
            <Button
                icon=IconName::Search
                minmal=true
            />
        }
    />
</div>
```

Use these new class in the `static/app.css` file
```css
.page-search{
    width: 30%;
}

.nav-button{
    margin-top: 30%;
}
```

## Add a new component

### Create the component

Create a new file with the name of the new component, let's call it "test",
with the following content:

```rust
use yew::prelude::*;

pub struct Test {}

impl Component for Test {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Test {}
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
                {"Test"}
            </div>
        }
    }
}

```

### Add the component to the tree

In `src/lib.rs`, declare the module for your new component
```rust
mod app;
mod nav_menu;
mod page;
mod test;
```

Now add the new component to `src/app.rs`:

Import the component crate
```rust
use crate::nav_menu::*;
use crate::page::*;
use crate::test::*;
use yew::prelude::*;
use yew_router::router::Router;
```

Add the component to the Router in the `<main>` tag
```rust
<div>
    <Router<AppMenu, ()>
        render=Router::render(|switch: AppMenu| {
            match switch {
                AppMenu::Page | AppMenu::Home => // correction à faire dans main
                    html! (<Page />),
                AppMenu::Test => html! (<Test />),
            }
        })
    />
</div>
```

In `src/nav_menu.rs`, add a test item to the menu

Create a new MenuItem
```rust
<div>
    <Menu>
        <MenuItem
            text={html!("Page")}
            href="#page"
            onclick=self.link
                .callback(|_| Msg::GoToMenu(AppMenu::Page))
        />
        <MenuItem
            text:{html!("Test")}
            href="#test"
            onclick=self.link
                .callback(|_| Msg::GoToMenu(AppMenu::Test))
        />
    </Menu>
    <Button
        class:"nav-button"
        fill=true
    >
        {"Ready?"}
    </Button>
</div>
```

Add the component to the AppMenu enum
```rust
#[derive(Debug, Copy, Clone, Switch)]
pub enum AppMenu {
    #[to = "/#page"]
    Page,
    #[to = "/#test]"
    Test,
    #[to = "/"]
    Home,
}
```

## Note

You can see all the changes of this tutorial in the
[demonstration pull request](https://github.com/Yozhgoor/yewprint-playground/pull/2)
