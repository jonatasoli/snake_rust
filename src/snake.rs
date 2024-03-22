use std::collections::HashMap;

use crate::{
    components::{Direction, GameEndEvent, Player, Position, Size},
    food::Food,
    grid::{GRID_HEIGHT, GRID_WIDTH},
};
use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE1_SEGMENT_COLOR: Color = Color::rgb(0.8, 0.0, 0.8); // <--
const SNAKE2_SEGMENT_COLOR: Color = Color::rgb(0., 0.8, 0.8); // <--

#[derive(Component)]
pub struct Head {
    direction: Direction,
}

#[derive(Component)]
pub struct Segment;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct Segments([Vec<Entity>; 2]);

#[derive(Event)]
pub struct GrowthEvent {
    pub player_id: u8,
}

#[derive(Default, Resource)]
pub struct LastTailPosition(Option<Position>);

impl Default for Head {
    fn default() -> Self {
        Self {
            direction: Direction::Up,
        }
    }
}

pub fn spawn_system(mut commands: Commands, mut segments: ResMut<Segments>) {
    *segments = Segments([
        spawn_entity_with_segment(&mut commands, 0),
        spawn_entity_with_segment(&mut commands, 1),
    ]);
}

pub fn spawn_segment_system(commands: &mut Commands, position: Position, player_id: u8) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: if player_id == 0 {
                    SNAKE1_SEGMENT_COLOR
                } else {
                    SNAKE2_SEGMENT_COLOR
                },
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Segment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

#[allow(clippy::needless_pass_by_value)]
pub fn movement_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut heads: Query<(&mut Head, &Player)>,
) {
    heads.iter_mut().for_each(|(mut head, player)| {
        let dir: Direction = if player.id() == 0 {
            if keyboard_input.pressed(KeyCode::KeyA) {
                Direction::Left
            } else if keyboard_input.pressed(KeyCode::KeyS) {
                Direction::Down
            } else if keyboard_input.pressed(KeyCode::KeyW) {
                Direction::Up
            } else if keyboard_input.pressed(KeyCode::KeyD) {
                Direction::Right
            } else {
                head.direction
            }
        } else if player.id() == 1 {
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                Direction::Left
            } else if keyboard_input.pressed(KeyCode::ArrowDown) {
                Direction::Down
            } else if keyboard_input.pressed(KeyCode::ArrowUp) {
                Direction::Up
            } else if keyboard_input.pressed(KeyCode::ArrowRight) {
                Direction::Right
            } else {
                head.direction
            }
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    });
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::cast_sign_loss)]
pub fn movement_system(
    segments: ResMut<Segments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_end_writer: EventWriter<GameEndEvent>, // <-- Adicionar EventWriter
    heads: Query<(Entity, &Head, &Player)>,
    mut positions: Query<(Entity, &Segment, &mut Position)>,
    game_end: Query<&GameEndEvent>, // <--Adicionar
) {
    let positions_clone: HashMap<Entity, Position> = positions
        .iter()
        .map(|(entity, _segment, position)| (entity, position.clone()))
        .collect();
    for (entity_id, head, Player { id }) in heads.iter() {
        let player_id = (*id) as usize;
        (*segments[player_id]).windows(2).for_each(|entity| {
            if let Ok((_, _segment, mut position)) = positions.get_mut(entity[1]) {
                if let Some(new_position) = positions_clone.get(&entity[0]) {
                    *position = new_position.clone();
                }
            };
        });

        if game_end.is_empty() {
            // <-- if verificando se houve um evento de fimd e jogo

            let _ = positions.get_mut(entity_id).map(|(_, _segment, mut pos)| {
                match &head.direction {
                    Direction::Left => {
                        pos.x -= 1;
                    }
                    Direction::Right => {
                        pos.x += 1;
                    }
                    Direction::Up => {
                        pos.y += 1;
                    }
                    Direction::Down => {
                        pos.y -= 1;
                    }
                };
                if pos.x < 0
                    || pos.y < 0
                    || pos.x as u16 >= GRID_WIDTH
                    || pos.y as u16 >= GRID_HEIGHT
                {
                    game_end_writer.send(GameEndEvent::GameOver); // <-- publicar evento
                }

                if positions_clone
                    .iter()
                    .filter(|(k, _)| k != &&entity_id)
                    .map(|(_, v)| v)
                    .any(|segment_position| &*pos == segment_position)
                {
                    game_end_writer.send(GameEndEvent::GameOver);
                }
            });
        }
        *last_tail_position = LastTailPosition(Some(
            positions_clone
                .get(segments[player_id].last().unwrap())
                .unwrap()
                .clone(),
        ));
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn eating_system(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<(&Position, &Player), With<Head>>,
) {
    for (head_pos, Player { id }) in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent { player_id: *id });
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn growth_system(
    mut commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<Segments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    growth_reader.read().for_each(|event| {
        let player_id = event.player_id as usize;
        if player_id < segments.len() {
            segments[player_id].push(spawn_segment_system(
                &mut commands,
                last_tail_position.0.clone().unwrap(),
                event.player_id,
            ));
        }
    });
}

#[allow(clippy::cast_possible_wrap)]
fn spawn_entity_with_segment(commands: &mut Commands, player_id: u8) -> Vec<Entity> {
    vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(Player { id: player_id })
            .insert(Head::default())
            .insert(Segment)
            .insert(Position {
                x: if player_id == 0 {
                    3
                } else {
                    (GRID_WIDTH - 3) as i16
                },
                y: 3,
            })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment_system(
            commands,
            Position {
                x: if player_id == 0 {
                    3
                } else {
                    (GRID_WIDTH - 3) as i16
                },
                y: 2,
            },
            player_id,
        ),
    ]
}

#[cfg(test)]
mod test {

    use crate::food::{self, Food};

    use super::*;

    #[test]
    fn entity_has_snake_head() {
        // 1 Inicialização do App
        let mut app = App::new();

        // 2 Adicionar o `spawn_snake` startup system
        app.insert_resource(Segments::default())
            .add_systems(Startup, spawn_system);

        // 3 Executar todos os sistemas pelo menos uma vez
        app.update();

        // 4 Fazer uma query por entidades que contenham o componente `SnakeHead`
        let mut query = app.world.query_filtered::<Entity, With<Head>>();

        // 5 Verificar se a contagem de componentes da query foi igual a 1
        assert_eq!(query.iter(&app.world).count(), 2); // Adicionar 2 snakes agora
    }

    #[test]
    fn snake_starts_moviment_up() {
        // Setup app
        let mut app = App::new();
        // Add startup system
        app.insert_resource(Segments::default())
            .add_systems(Startup, spawn_system);
        // Run systems
        app.update();
        let mut query = app.world.query::<&Head>();
        let head = query.iter(&app.world).next().unwrap();
        assert_eq!(head.direction, Direction::Up);
    }

    #[test]
    fn snake_head_has_moved_up() {
        // Setup
        let mut app = App::new();
        // Adicionar positions para os dois players
        let p1_default_position = Position { x: 3, y: 4 };
        #[cfg(debug_assertions)]
        let p2_default_position = Position { x: 7, y: 4 };
        #[cfg(not(debug_assertions))]
        let p2_default_position = Position { x: 17, y: 4 };

        // Adicionando sistemas
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_systems(Startup, spawn_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, movement_input_system.before(movement_system));

        // Adicionando inputs de `KeyCode`s
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyW);
        app.insert_resource(input);

        // Executando sistemas pelo menos uma vez
        app.update();

        // Mudar o assert
        let mut query = app.world.query::<(&Head, &Position, &Player)>();
        query
            .iter(&app.world)
            .for_each(|(head, position, Player { id })| {
                if id == &0 {
                    assert_eq!(&p1_default_position, position);
                } else {
                    assert_eq!(&p2_default_position, position);
                }
                assert_eq!(head.direction, Direction::Up);
            })
    }
    #[test]
    fn snake_head_moves_up_and_right() {
        // Setup
        let mut app = App::new();
        // Mudar as positions para 2 players
        let p1_up_position = Position { x: 3, y: 4 };
        #[cfg(debug_assertions)]
        let p2_up_position = Position { x: 7, y: 4 };
        #[cfg(not(debug_assertions))]
        let p2_up_position = Position { x: 17, y: 4 };

        // Adiciona systemas
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_systems(Startup, spawn_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, movement_input_system.before(movement_system));

        // Testa movimento para cima
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyW);
        app.insert_resource(input);
        app.update();

        // Mudar o assert
        let mut query = app.world.query::<(&Head, &Position, &Player)>();
        query
            .iter(&app.world)
            .for_each(|(_head, position, Player { id })| {
                if id == &0 {
                    assert_eq!(&p1_up_position, position);
                } else {
                    assert_eq!(&p2_up_position, position);
                }
            });

        // Adicionar o movimento para 2 players
        let p1_up_right_position = Position { x: 4, y: 4 };
        #[cfg(debug_assertions)]
        let p2_up_right_position = Position { x: 7, y: 5 };
        #[cfg(not(debug_assertions))]
        let p2_up_right_position = Position { x: 17, y: 5 };

        // Testa movimento para direita
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyD);
        app.insert_resource(input);
        app.update();

        // Mudar o assert
        let mut query = app.world.query::<(&Head, &Position, &Player)>();
        query
            .iter(&app.world)
            .for_each(|(head, position, Player { id })| {
                if id == &0 {
                    assert_eq!(&p1_up_right_position, position);
                    assert_eq!(head.direction, Direction::Right);
                } else {
                    assert_eq!(&p2_up_right_position, position);
                    assert_eq!(head.direction, Direction::Up);
                }
            })
    }

    #[test]
    fn snake_head_moves_down_and_left() {
        // Setup
        let mut app = App::new();
        // Adicionar a positions para 2 players
        let down_left_position = Position { x: 2, y: 4 };
        #[cfg(debug_assertions)]
        let p2_up_position = Position { x: 7, y: 5 };
        #[cfg(not(debug_assertions))]
        let p2_up_position = Position { x: 17, y: 5 };

        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_systems(Startup, spawn_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, movement_input_system.before(movement_system));

        // Movimenta para baixo
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyS);
        app.insert_resource(input);
        app.update();

        // Movimenta para esquerda
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyA);
        app.insert_resource(input);
        app.update();

        // Mudar o Assert
        let mut query = app.world.query::<(&Head, &Position, &Player)>();
        query
            .iter(&app.world)
            .for_each(|(head, position, Player { id })| {
                if id == &0 {
                    assert_eq!(&down_left_position, position);
                    assert_eq!(head.direction, Direction::Left);
                } else {
                    assert_eq!(&p2_up_position, position);
                    assert_eq!(head.direction, Direction::Up);
                }
            })
    }

    #[test]
    fn snake_cannot_start_moving_down() {
        // Setup
        let mut app = App::new();
        // Adicionar positions para 2 players
        let p1_down_left_position = Position { x: 3, y: 4 };
        #[cfg(debug_assertions)]
        let p2_down_left_position = Position { x: 7, y: 4 };
        #[cfg(not(debug_assertions))]
        let p2_down_left_position = Position { x: 17, y: 4 };

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>() // <--
            .add_systems(Startup, spawn_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, movement_input_system.before(movement_system));

        // Move down
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyS);
        app.insert_resource(input);
        app.update();

        // Mudar o assert
        let mut query = app.world.query::<(&Head, &Position, &Player)>();
        query
            .iter(&app.world)
            .for_each(|(_head, position, Player { id })| {
                if id == &0 {
                    assert_eq!(&p1_down_left_position, position);
                } else {
                    assert_eq!(&p2_down_left_position, position);
                }
            })
    }

    #[test]
    fn entity_snake_has_two_segments() {
        // Setup app
        let mut app = App::new();

        // Adicionar sistema de spawn e recurso com segmentos
        app.insert_resource(Segments::default())
            .add_systems(Startup, spawn_system);

        // Executar sistema
        app.update();

        // Buscar todas entidades com componente `Segment`
        let mut query = app.world.query_filtered::<Entity, With<Segment>>();
        assert_eq!(query.iter(&app.world).count(), 4); // <-- Alterar a quantidade pra 2 players
    }

    #[test]
    fn snake_segment_has_followed_head() {
        // Setup
        let mut app = App::new();
        // Mudar valores dos parametros
        let new_position_head_right = Position { x: 4, y: 3 };
        let new_position_segment_right = Position { x: 3, y: 3 };

        // Adiciona os systemas
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_systems(Startup, spawn_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, movement_input_system.before(movement_system));

        // adiciona resource apertando a tecla D, movimento para direita
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyD);
        app.insert_resource(input);

        // executa sistemas
        app.update();

        // Mudar a query agora usando o players
        let mut query = app.world.query::<(&Head, &Position, &Player)>();
        query
            .iter(&app.world)
            .filter(|(_, _, player)| player.id == 0)
            .for_each(|(head, position, _)| {
                // garante que nova posição da cabeça é esperada:
                assert_eq!(&new_position_head_right, position);
                // garante que nova direção é para direita:
                assert_eq!(head.direction, Direction::Right);
            });

        let mut query = app
            .world
            .query_filtered::<(&Head, &Position, &Player), Without<Head>>(); // <-- Alterar adiconando o player
        query
            .iter(&app.world)
            .filter(|(_, _, player)| player.id == 0) // <-- Adicionar um filtro
            .for_each(|(head, position, _)| {
                // garante que nova posição do segmento é esperada:
                assert_eq!(&new_position_segment_right, position);
                assert_eq!(head.direction, Direction::Right);
            });

        // NOVAS POSIÇÕES ESPERADAS
        let new_position_head_up = Position { x: 4, y: 4 }; // <-- Mudar
        let new_position_segment_up = Position { x: 4, y: 3 }; // <-- Mudar

        // adiciona resource apertando a tecla W, movimento para cima
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyW); // <--
        app.insert_resource(input);

        // executa sistemas de novo
        app.update();

        let mut query = app.world.query::<(&Head, &Position, &Player)>();
        query
            .iter(&app.world)
            .for_each(|(head, position, Player { id })| {
                if id == &0 {
                    // garante que nova posição da cabeça é esperada:
                    assert_eq!(&new_position_head_up, position);
                    // garante que nova direção da cabeça é esperada:
                    assert_eq!(head.direction, Direction::Up);
                }
            });

        let mut query = app
            .world
            .query_filtered::<(&Segment, &Position, &Player), Without<Head>>();
        query
            .iter(&app.world)
            .for_each(|(_segment, position, Player { id })| {
                if id == &0 {
                    // garante que nova posição do segmento é esperada:
                    assert_eq!(&new_position_segment_up, position);
                }
            })
    }

    #[test]
    fn snake_grows_when_eating() {
        // Setup
        let mut app = App::new();

        // sistemas
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_event::<GrowthEvent>()
            .add_systems(Startup, spawn_system)
            .add_systems(Update, food::spawn_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, eating_system.after(movement_system))
            .add_systems(Update, growth_system.after(eating_system));

        // update de configuração
        app.update();

        let mut query = app.world.query::<(&Segment, &Position)>();
        assert_eq!(query.iter(&app.world).count(), 4); // <-- Alterar para 2 players
        let mut query = app.world.query::<(&Food, &Position)>();
        assert_eq!(query.iter(&app.world).count(), 1);

        // update de execução
        app.update();

        let mut query = app.world.query::<(&Segment, &Position)>();
        assert_eq!(query.iter(&app.world).count(), 5); // <-- Alterar pra 2 players
    }
}
