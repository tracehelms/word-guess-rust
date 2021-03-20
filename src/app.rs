use crate::components::github_banner::GitHubBanner;
use crate::game::Game;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    state: Game,
}

pub enum Msg {
    Guess,
    SetGuess(String),
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            state: Game::new(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Guess => self.state.guess(),
            Msg::SetGuess(val) => self.state.set_guess(val),
            Msg::Nope => (),
        }
        true
    }

    fn view(&self) -> Html {
        let value = match &self.state.guess_value {
            None => String::from(""),
            Some(v) => v.to_string(),
        };

        let maybe_show_guessing_form = move || -> Html {
            if self.state.game_result.is_some() {
                html! {}
            } else {
                html! {
                    <>
                        <div>
                            <label for="guess">{"Guess a letter:"}</label>
                            <input
                                name="guess"
                                value=value
                                oninput=self.link.callback(|e: InputData| Msg::SetGuess(e.value))
                                onkeypress=self.link.callback(|e: KeyboardEvent| {
                                    if e.key() == "Enter" { Msg::Guess } else { Msg::Nope }
                                })
                                />
                        </div>
                        <div>
                            <button onclick=self.link.callback(|_| Msg::Guess)>{"Guess"}</button>
                        </div>
                    </>
                }
            }
        };

        html! {
            <>
                <div class="container">
                    <div class="word-area">
                        <h1>{"Guess the word!"}</h1>
                        <h2>
                            {"Your word: "}
                            <span class="the-word">{self.state.word_with_guesses()}</span>
                        </h2>
                    </div>

                    <div class="guesses-area">
                        <p>{"You already guessed: "}{&self.state.already_guessed()}</p>
                        <p>{"Guesses left: "}{&self.state.guesses_left}</p>
                    </div>

                    <div class="guess-area">
                        <p>{&self.state.error_message}</p>
                        {maybe_show_guessing_form()}
                    </div>

                    <h2>{self.state.game_over_message()}</h2>
                </div>
                <GitHubBanner />
            </>
        }
    }
}
