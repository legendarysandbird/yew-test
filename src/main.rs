use yew::prelude::*;

enum Msg {
    AddOne,
    SubOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::SubOne => {
                if self.value > 0 {
                    self.value -= 1;
                    return true;
                }

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages,
        // etc to the component
        let link = ctx.link();
        html! {
            <body>
                <div>
                    <h1> {"Brandon's Amazing Website v1."} {self.value} </h1>
                    <p> {"I can do a lot in Yew. Pretty cool, huh?"} </p>
                </div>
                <div>
                    <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                    <button onclick={link.callback(|_| Msg::SubOne)}>{ "-1" }</button>
                </div>
            </body>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
