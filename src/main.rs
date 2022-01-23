mod items;
mod profile;
mod simulation;

use web_sys::HtmlSelectElement;
use yew::prelude::*;

enum Msg {
    AddOne,
    OnClassLevel(Option<String>),
    OnClassQuestLevel(Option<String>),
    OnItemSelect(items::Item),
    OnItemLevel(Option<String>),
    OnItemDurability(Option<String>),
    OnItemHQ(Option<String>),
    OnItemProgress(Option<String>),
    GetActions,
    Simulate,
}

struct Model {
    value: i64,

    alchemist_level_input: String,
    alchemist_class_quest_level_input: String,

    alchemist_stats: profile::CraftingClassStats,
    alchemist_abilities: Vec<profile::Ability>,

    item_level_input: String,
    item_durability_input: String,
    item_hq_input: String,
    item_progress_input: String,

    item: items::ItemStats,

    simulation_results: Result<simulation::SimulationRun, ()>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            alchemist_level_input: String::from("90"),
            alchemist_class_quest_level_input: String::from("90"),

            alchemist_stats: profile::CraftingClassStats {
                level: 90,
                class_quest_level: 90,
                attributres: profile::BuffedCraftingAttributes::Final(
                    profile::CraftingAttributes {
                        cp: 662,
                        control: 3556,
                        craftmanship: 3226,
                    },
                ),
            },
            alchemist_abilities: vec![],

            item_level_input: String::from("1"),
            item_durability_input: String::from("1"),
            item_hq_input: String::from("1"),
            item_progress_input: String::from("1"),

            item: items::ItemStats {
                level: 1,
                initial_durability: 1,
                target_quality: 1,
                required_progress: 1,
            },

            simulation_results: Err(()),
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
            Msg::OnClassLevel(l) => {
                match l {
                    Some(lv) => match lv.parse() {
                        Ok(d) => {
                            self.alchemist_stats.level = d;
                            self.alchemist_level_input = lv;
                        }
                        Err(e) => {
                            log::error!("Bad value! {:?}", e);
                        }
                    },
                    None => {
                        self.alchemist_level_input = String::from("1");
                        self.alchemist_stats.level = 1;
                    }
                }
                true
            }
            Msg::OnClassQuestLevel(l) => {
                match l {
                    Some(lv) => match lv.parse() {
                        Ok(d) => {
                            self.alchemist_stats.class_quest_level = d;
                            self.alchemist_class_quest_level_input = lv;
                        }
                        Err(e) => {
                            log::error!("Bad value! {:?}", e);
                        }
                    },
                    None => {
                        self.alchemist_class_quest_level_input = String::from("1");
                        self.alchemist_stats.class_quest_level = 1;
                    }
                }
                true
            }
            Msg::OnItemSelect(l) => {
                self.item = l.stats;
                self.item_durability_input = self.item.initial_durability.to_string();
                self.item_hq_input = self.item.target_quality.to_string();
                self.item_level_input = self.item.level.to_string();
                self.item_progress_input = self.item.required_progress.to_string();
                true
            }
            Msg::OnItemLevel(l) => {
                match l {
                    Some(lv) => match lv.parse() {
                        Ok(d) => {
                            self.item.level = d;
                            self.item_level_input = lv;
                        }
                        Err(e) => {
                            log::error!("Bad value! {:?}", e);
                        }
                    },
                    None => {
                        self.item_level_input = String::from("1");
                        self.item.level = 1;
                    }
                }
                true
            }
            Msg::OnItemDurability(l) => {
                match l {
                    Some(lv) => match lv.parse() {
                        Ok(d) => {
                            self.item.initial_durability = d;
                            self.item_durability_input = lv;
                        }
                        Err(e) => {
                            log::error!("Bad value! {:?}", e);
                        }
                    },
                    None => {
                        self.item_durability_input = String::from("40");
                        self.item.initial_durability = 40;
                    }
                }
                true
            }
            Msg::OnItemHQ(l) => {
                match l {
                    Some(lv) => match lv.parse() {
                        Ok(d) => {
                            self.item.target_quality = d;
                            self.item_hq_input = lv;
                        }
                        Err(e) => {
                            log::error!("Bad value! {:?}", e);
                        }
                    },
                    None => {
                        self.item_hq_input = String::from("100");
                        self.item.target_quality = 100;
                    }
                }
                true
            }
            Msg::OnItemProgress(l) => {
                match l {
                    Some(lv) => match lv.parse() {
                        Ok(d) => {
                            self.item.required_progress = d;
                            self.item_progress_input = lv;
                        }
                        Err(e) => {
                            log::error!("Bad value! {:?}", e);
                        }
                    },
                    None => {
                        self.item_progress_input = String::from("100");
                        self.item.required_progress = 100;
                    }
                }
                true
            }
            Msg::GetActions => {
                self.alchemist_abilities =
                    profile::get_all_abilities_at_level(&self.alchemist_stats);
                true
            }
            Msg::Simulate => {
                let final_stats = self.alchemist_stats.attributres.final_attributes();

                let constants = simulation::SimulationConstants {
                    class_level: self.alchemist_stats.level,
                    craftmanship: final_stats.craftmanship,
                    control: final_stats.control,
                    cp: final_stats.cp,
                    available_abilities: self.alchemist_abilities.clone(), // TODO: Make it not have to clone

                    item: self.item,

                    max_steps: 15,
                    accept_first_of_length: 8,
                };
                self.simulation_results = simulation::simulate(
                    &constants,
                    simulation::ConditionSimulation::AlwaysNormal,
                    simulation::RandomActionSuccessChance::AlwaysFail,
                );
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let abilities = self.alchemist_abilities.clone();
        let simulation_results = self.simulation_results.clone();

        let alchemist_items = items::get_craftable_items_for_class_at_level(
            profile::CraftingClass::Alchemist,
            &self.alchemist_stats,
        );

        let alchemist_copy = alchemist_items.clone();
        let item_change = link.batch_callback(move |e: Event| {
            let input = e.target_dyn_into::<HtmlSelectElement>();
            input.map(|input| {
                let maybe_item = alchemist_copy.get(input.selected_index() as usize);
                Msg::OnItemSelect(*maybe_item.unwrap_or(&items::Item {
                    name: "INALID",
                    stats: items::ItemStats {
                        level: 1,
                        initial_durability: 1,
                        target_quality: 1,
                        required_progress: 1,
                    },
                }))
            })
        });
        html! {
            <div>
                <label for="class-level">
                    { "Alchemist Class Level" }
                    <input type="number" id="class-level"
                        value={self.alchemist_level_input.clone()}
                        oninput={link.callback(|e : InputEvent| Msg::OnClassLevel(e.data()))}
                    />
                </label>
                <br/>
                <label for="class-quest-level">{ "Alchemist Class Quest Level" }
                    <input type="number" id="class-quest-level"
                        value={self.alchemist_class_quest_level_input.clone()}
                        oninput={link.callback(|e : InputEvent| Msg::OnClassQuestLevel(e.data()))}
                    />
                </label>
                <br/>
                <label for="item-selector">{ "Item" }
                    <select id="item-selector" onchange={item_change}>
                    {
                        alchemist_items.into_iter().map(|ref item| {
                            html!{<option value={item.name.to_string()}> { item.name } </option>}
                        }).collect::<Html>()
                    }
                    </select>
                </label>
                <br/>
                <label for="item-level">{ "Item Level" }
                    <input type="number" id="item-level"
                        value={self.item_level_input.clone()}
                        oninput={link.callback(|e : InputEvent| Msg::OnItemLevel(e.data()))}
                    />
                </label>
                <br/>
                <label for="item-durability">{ "Item Intial Durability" }
                    <input type="number" id="item-durability"
                        value={self.item_durability_input.clone()}
                        oninput={link.callback(|e : InputEvent| Msg::OnItemDurability(e.data()))}
                    />
                </label>
                <br/>
                <label for="item-progress">{ "Item Progress Requirement" }
                    <input type="number" id="item-progress"
                        value={self.item_progress_input.clone()}
                        oninput={link.callback(|e : InputEvent| Msg::OnItemProgress(e.data()))}
                    />
                </label>
                <br/>
                <label for="item-hq">{ "Item HQ Requirement" }
                    <input type="number" id="item-hq"
                        value={self.item_hq_input.clone()}
                        oninput={link.callback(|e : InputEvent| Msg::OnItemHQ(e.data()))}
                    />
                </label>
                <br/>
                <button onclick={link.callback(|_| Msg::GetActions)}>{ "Get Actions" }</button>
                <br/>
                <div id="abilities">
                {
                    abilities.into_iter().map(|ref ability| {
                        html!{<div key={ability.name.to_string()}> { format!("Ability: {}", ability.name) } </div>}
                    }).collect::<Html>()
                }
                </div>
                <br/>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
                <br/>
                <button onclick={link.callback(|_| Msg::Simulate)}>{ "Simulate" }</button>
                <br/>
                <div id="simulation-outcome">
                {
                    match &simulation_results {
                        Ok(res) => {
                            html!{<div>
                                <div>
                                    {
                                        res.steps.iter().map(|step| {
                                            html!{<div> { format!("Ability: {} - Condition: {:?} - Action Success: {:?}", step.ability, step.condition, step.action_success) } </div>}
                                        }).collect::<Html>()
                                    }
                                </div>
                                <div>
                                    {
                                        html!{<div> { format!("Final Quality: {}", res.quality) } </div>}
                                    }
                                </div>
                            </div>}
                        }
                        Err(()) => {
                            html!({ "No possible outcome" })
                        }
                    }
                }
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
