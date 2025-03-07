use engine::edit_global;
use engine::{Color, Entity, EntityType, GameInput, GameState, Vector2, Vector3};
use fermium::{
    events::SDL_Event,
    rect::SDL_Rect,
    renderer::{SDL_RenderFillRect, SDL_Renderer, SDL_SetRenderDrawColor},
};
use std::sync::{Arc, Mutex};

fn test_gradient(renderer: *mut SDL_Renderer) {
    unsafe {
        let rect = SDL_Rect {
            x: 50,
            y: 50,
            w: 50,
            h: 50,
        };
        let color: Color = renderer.into();
        SDL_SetRenderDrawColor(renderer, 155, 20, 134, 255);
        SDL_RenderFillRect(renderer, &rect);
        color.renderer(renderer);
    }
}

fn move_player(game_state: &mut GameState) {
    let mut pos: Vector2 = game_state.entities[0].clone().into();
    pos.x += 4.0;
    pos.y += 4.0;

    if pos.x > 500.0 {
        if game_state.entities[0].position.z == 0.0 {
            game_state.update_entity_z(0, 3.0).unwrap();
        } else {
            game_state.update_entity_z(0, 0.0).unwrap();
        }

        pos.x = 200.0;
        pos.y = 200.0;
    }
    game_state.entities[0].position.xy = pos.clone();
}

#[no_mangle]
pub fn decide_input(event: SDL_Event) -> engine::GameInput {
    return GameInput {
        ..Default::default()
    };
}

#[no_mangle]
pub fn init(game_state_arc: Arc<Mutex<GameState>>) -> bool {
    edit_global!(game_state, immut_game_state, game_state_arc, {
        let player_entity_id = game_state.add_entity(Entity {
            etype: EntityType::Player,
            exists: true,
            position: Vector3 {
                xy: Vector2 { x: 200.0, y: 200.0 },
                z: 0.0,
            },
            position_delta: Vector3 {
                xy: Vector2 { x: 0.0, y: 0.0 },
                z: 0.0,
            },
            direction: 0,
            width: 50.0,
            height: 50.0,
        });
        let debug_entity_id = game_state.add_entity(Entity {
            etype: EntityType::DebugText,
            exists: true,
            position: Vector3 {
                xy: Vector2 {
                    x: (immut_game_state.window.width - 500) as f32,
                    y: (immut_game_state.window.height - 30) as f32,
                },
                z: 0.0,
            },
            position_delta: Vector3 {
                xy: Vector2 { x: 0.0, y: 0.0 },
                z: 0.0,
            },
            direction: 0,
            width: 500.0,
            height: 0.0,
        });
        game_state.add_entity(Entity {
            etype: EntityType::Rect(Color {
                r: 1.0,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            }),
            exists: true,
            position: Vector3 {
                xy: Vector2 { x: 300.0, y: 300.0 },
                z: 1.0,
            },
            width: 100.0,
            height: 100.0,
            ..Default::default()
        });
        game_state.add_entity(Entity {
            etype: EntityType::Rect(Color {
                r: 0.1,
                g: 0.4,
                b: 0.1,
                a: 0.3,
            }),
            exists: true,
            position: Vector3 {
                xy: Vector2 { x: 320.0, y: 320.0 },
                z: 1.2,
            },
            width: 100.0,
            height: 100.0,
            ..Default::default()
        });
        game_state.add_entity(Entity {
            etype: EntityType::Rect(Color {
                r: 0.1,
                g: 0.0,
                b: 0.1,
                a: 0.3,
            }),
            exists: true,
            position: Vector3 {
                xy: Vector2 { x: 340.0, y: 350.0 },
                z: 4.0,
            },
            width: 40.0,
            height: 100.0,
            ..Default::default()
        });
    });
    true
}
#[no_mangle]
pub fn update_and_render(
    renderer: *mut SDL_Renderer,
    game_state_arc: Arc<Mutex<GameState>>,
) -> bool {
    edit_global!(game_state, game_state_arc, {
        test_gradient(renderer);
        move_player(&mut game_state);
        engine::render(renderer, &mut game_state);
    });
    true
}
