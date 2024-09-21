use gloo_net::http::Request;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::Renderer;

pub struct Payment {
    id: Option<AttrValue>,
}

pub enum Msg {
    Pay(AttrValue),
    TryPay,
}

impl Component for Payment {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { id: None }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let pago = self
            .id
            .as_ref()
            .map(|id| format!("Pago {}!", id))
            .unwrap_or("Aguardando Pagamento....".to_string());

        html! {
            <div>
                <button onclick={ctx.link().callback(|_|Msg::TryPay)}>{ "Pagar" }</button>
                <h2>{ pago }</h2>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Pay(pay) => {
                self.id = Some(pay);
                true
            }
            Msg::TryPay => {
                let pay_cb = ctx.link().callback(Msg::Pay);
                pay(pay_cb);
                false
            }
        }
    }
}

fn pay(pay_cb: Callback<AttrValue>) {
    let id = 1;

    spawn_local(async move {
        let url = format!("transaction/{}", id);
        let response = Request::post(url.as_str())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        pay_cb.emit(AttrValue::from(response));
    });
}

fn main() {
    Renderer::<Payment>::new().render();
}
