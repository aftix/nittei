use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WhatMsg {}

pub struct What {
    _link: ComponentLink<Self>,
}

impl Component for What {
    type Message = WhatMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section id="home-what">
                <h1>{"What"}</h1>
                <p>{"Non aliquam debitis non. Corporis id quibusdam aperiam consequuntur porro. Corporis quo sunt est qui. Aut eaque ipsam ipsa rem accusantium odio inventore. Nesciunt ut veritatis pariatur labore laborum. Harum incidunt accusantium et sed. Perferendis qui repudiandae facilis quo voluptate est. Minus et iusto est. Quo maxime explicabo nostrum ipsum. Commodi aut ut ut fugit. Et voluptate asperiores omnis voluptatibus. Placeat et rem eum eum sunt veniam. Eum cupiditate quos natus. Voluptatem dolorem dolorem in incidunt. Eligendi sapiente et unde odio. Sint aliquid iusto molestiae numquam laudantium ad facere dolores. At cumque eius voluptates omnis aliquam fuga doloremque occaecati. Id qui tenetur perferendis necessitatibus omnis et. Sit et inventore eum eius. Animi minus iure eius earum quos qui quibusdam soluta. Officia voluptatem veritatis fugit illo neque."}</p>
            </section>
        }
    }
}
