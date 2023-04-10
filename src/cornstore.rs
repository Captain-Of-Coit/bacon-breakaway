use bevy::prelude::*;
use crate::{states, player::PiggieStats};
use belly::prelude::*;

pub struct CornstorePlugin;

impl Plugin for CornstorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<StartGameEvent>()
        .add_event::<PurchaseEvent>()
        .add_system(setup_cornstore.in_schedule(OnEnter(states::GameState::CornStore)))

        .add_system(start_game_event_handler.run_if(in_state(states::GameState::CornStore)))

        .add_system(feed_event_handler.run_if(in_state(states::GameState::CornStore)))

        // .add_system(update_stats.run_if(in_state(states::GameState::CornStore)))
        // .add_system(start_game_event_handler.run_if(in_state(states::GameState::CornStore)))
        // .add_system(quit_game_event_handler.run_if(in_state(states::GameState::MainMenu)))
        // .add_system(update_mainmenu.run_if(in_state(states::GameState::MainMenu)))
        ;
    }
}

struct StartGameEvent {}

enum PurchaseType {
    Apple,
    Donut,
    Gym,
    Nil,
}

struct PurchaseEvent (PurchaseType);

fn start_game_event_handler(
    mut events: EventReader<StartGameEvent>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    query: Query<Entity, With<Transform>>,
    mut commands: Commands
) {
    for _event in events.iter() {
        // info!("StartGameEvent");
        for e in query.iter() {
            commands.entity(e).despawn_recursive();
        }
        next_state.set(crate::states::GameState::InGame);
    }
}

fn feed_event_handler(
    mut events: EventReader<PurchaseEvent>,
    mut piggiestats: ResMut<PiggieStats>
) {
    // println!(".");
    for event in events.iter() {
        // info!("FeedEvent");
        // println!("Feeding");
        match event.0 {
            PurchaseType::Apple => {
                // TODO also check if we have enough energy already
                if piggiestats.corn > 10 {
                    piggiestats.corn = piggiestats.corn - 10;
                    piggiestats.energy = piggiestats.energy + 10.0;
                }
                // piggiestats.fatness = piggiestats.energy + 10.0;
            },
            PurchaseType::Donut => {
                if piggiestats.corn > 15 {
                    piggiestats.corn = piggiestats.corn - 15;
                    piggiestats.energy = piggiestats.energy + 25.0;
                    piggiestats.fatness = piggiestats.fatness + 10.0;
                }
            }
            PurchaseType::Gym => {
                if piggiestats.energy > 30.0 {
                    piggiestats.energy = piggiestats.energy - 30.0;
                    piggiestats.fatness = piggiestats.fatness - 2.0;
                    piggiestats.stamina = piggiestats.stamina + 0.02;
                }
            }
            // Hack to get UI to update
            PurchaseType::Nil => {
                piggiestats.energy = piggiestats.energy;
                piggiestats.fatness = piggiestats.fatness;
                piggiestats.stamina = piggiestats.stamina;
                piggiestats.corn = piggiestats.corn;
            }
        }
        if piggiestats.energy > 100.0 {
            piggiestats.energy = 100.0;
        }
        if piggiestats.fatness > 100.0 {
            piggiestats.fatness = 100.0;
        }
        if piggiestats.fatness < 0.0 {
            piggiestats.fatness = 0.0;
        }
        if piggiestats.stamina > 1.0 {
            piggiestats.stamina = 1.0;
        }
    }
}

fn setup_cornstore(
    mut commands: Commands,
    mut ev_purchase: EventWriter<PurchaseEvent>,
    // image_assets: Res<crate::assets::ImageAssets>,
    // asset_server: Res<AssetServer>
    image_assets: Res<crate::assets::ImageAssets>,
    windows: Query<&Window>,
    assets: Res<Assets<Image>>,
) {
    // println!("In corn store");
    

    // let donut_img = image_assets.donut.clone();
    let apple_img = image_assets.apple.clone();

    // let apple_img: Handle<Image> = image_assets.apple.clone().cast_weak();
    // let apple_img: Handle<Image> = asset_server.load("images/apple.png");

    commands.spawn(
        crate::utils::create_bg(
            &image_assets.cornstore_bg.clone(),
            assets,
            windows
        )
    );

    commands.spawn(Camera2dBundle::default());

    let img = commands.spawn_empty().id();
    
    commands.add(eml! {
        <body>
            <div s:align-items="center" s:justify-content="center" s:width="100%" s:height="100%" s:flex-direction="column" s:color="white" s:padding="50px">
                <div s:width="100%" s:align-items="center" s:justify-content="center" s:width="100%" s:padding="1%" s:color="white" s:background-color="xyellow" s:font-size="36px">
                    "Welcome to the corn store of your dreams"
                    <img {img} src=apple_img/>
                    <img {img} src="images/apple.png" mode="fit"/>
                </div>
                <br/>
                <div s:width="100%" s:height="90%" s:flex-direction="row" s:background-color="xblue">
                    // Left sidebar
                    <div s:width="50%" s:height="100%" s:flex-direction="column" s:background-color="xred" s:color="white">
                        "Corn: " {from!(PiggieStats:corn | fmt.s("{s}"))}

                        "Energy "{from!(PiggieStats:energy | fmt.s("{s:0.2}"))}
                        <progressbar s:width="400px" maximum=100. bind:value=from!(PiggieStats:energy)/>

                        "Fatness "{from!(PiggieStats:fatness | fmt.s("{s:0.2}"))}
                        <progressbar s:width="400px" maximum=100. bind:value=from!(PiggieStats:fatness)/>

                        "Stamina "{from!(PiggieStats:stamina | fmt.s("{s:0.2}"))}
                        <progressbar s:width="400px" maximum=1. bind:value=from!(PiggieStats:stamina)/>

                        

                        // "Elapsed seconds: "{from!(Stats:energy)}
                        // <label bind:value=from!(stats.energy | fmt.s("{s:0.2}"))/>
                        // <label {piggiestats} with=PiggieStats/>
                        // "Energy:"
                        // 
                        // "Fatness:"
                        // <progressbar s:width="400px" maximum=100. bind:value=from!(Time:elapsed_seconds())/>
                    </div>
                    // Right sidebar
                    <div s:width="50%" s:height="100%" s:flex-direction="column" s:background-color="xgreen">
                        // <img src=apple_img.clone()/>
                        <div s:flex-direction="row" s:color="black" s:align-items="center" s:color="white">
                            // <img src=apple_img.clone()/>
                            // <button c:green on:press=|ctx| ctx.send_event(PurchaseEvent(PurchaseType::Apple)) >    
                            //     "Apple (-10 corn, +10 energy)" 
                            // </button>
                            <button c:green on:press=|ctx| ctx.send_event(PurchaseEvent(PurchaseType::Apple)) s:width="100px" >    
                                "Apple"
                            </button>
                            "Gain a bit of energy while staying healthy. Only for 10 corns!"
                        </div>
                        <div s:flex-direction="row" s:color="black" s:align-items="center" s:color="white">
                            // <button c:green on:press=|ctx| ctx.send_event(PurchaseEvent(PurchaseType::Donut)) >
                            //     "Donut (-15 corn, +25 energy, +25 fatness)" 
                            // </button>
                            <button c:green on:press=|ctx| ctx.send_event(PurchaseEvent(PurchaseType::Donut)) s:width="100px">
                                "Donut" 
                            </button>
                            "Bulky but sweet! For 15 corns get the extra energy you need."
                        </div>
                        <div s:flex-direction="row" s:color="black" s:align-items="center" s:color="white">
                            // <button c:green on:press=|ctx| ctx.send_event(PurchaseEvent(PurchaseType::Gym)) >
                            //     "Hit the gym (-30 energy, +2 stamina, -2 fatness)" 
                            // </button>
                            <button c:green on:press=|ctx| ctx.send_event(PurchaseEvent(PurchaseType::Gym)) s:width="100px">
                                "Hit the gym" 
                            </button>
                            "Spend some energy but loose weight and keep your power!"
                        </div>
                        <div s:margin-top="50px">
                            <button c:lightblue on:press=|ctx| ctx.send_event(StartGameEvent {}) >
                                "Run for your life!"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </body>
    });
    commands.add(StyleSheet::parse(
        r#"
        body > img {
            width: 150px;
            height: 150px;
        }
        .green .button-foreground {
            background-color: green;
            color: white;
        }
        .red .button-foreground {
            background-color: red;
            color: white;
        }
        .lightblue .button-foreground {
            background-color: lightblue;
            color: black;
        }
        button:hover > span > .button-foreground {
            background-color: white;
            color: black;
        }
    "#,
    ));
    // Hack to update UI
    ev_purchase.send(PurchaseEvent(PurchaseType::Nil));
}