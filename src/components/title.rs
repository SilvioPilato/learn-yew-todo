use yew::ShouldRender;
use yew::Html;
use yew::html;
use yew::Component;
use yew::ComponentLink;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub text: String,
}


pub struct Title {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Title {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self { 
        Title { link, props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender { 
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender  {
        if self.props != props {
            self.props = props;
            return true;
        } 
        return false;
    }

    fn view(&self) -> Html { 
        html!{
            <h1 class="title">{&self.props.text}</h1>
        }
    }
}