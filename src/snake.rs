use std::collections::HashMap;

use crate::{
    components::{Direction, GameEndEvent, Position, Size},
    food::Food,
    grid::{GRID_HEIGHT, GRID_WIDTH},
};
use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.8, 0.0, 0.8);

#[derive(Component)]
pub struct Head {
    direction: Direction,
}

#[derive(Component)]
pub struct Segment;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct Segments(Vec<Entity>);

#[derive(Event)]
pub struct GrowthEvent;

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
    *segments = Segments(vec![
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
            .insert(Head::default())
            .insert(Segment)
            .insert(Position { x: 5, y: 5 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment_system(commands, Position { x: 5, y: 4 }), // <-- novo segmento
    ]);
}

pub fn spawn_segment_system(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
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
pub fn movement_input_system(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut Head>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::A) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::S) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::W) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::D) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn movement_system(
    segments: ResMut<Segments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_end_writer: EventWriter<GameEndEvent>, // <-- Adicionar EventWriter
    mut heads: Query<(Entity, &Head)>,
    mut positions: Query<(Entity, &Segment, &mut Position)>,
    game_end: Query<&GameEndEvent>, // <--Adicionar
) {
    let positions_clone: HashMap<Entity, Position> = positions
        .iter()
        .map(|(entity, _segment, position)| (entity, position.clone()))
        .collect();
    if let Some((id, head)) = heads.iter_mut().next() {
        (*segments).windows(2).for_each(|entity| {
            if let Ok((_, _segment, mut position)) = positions.get_mut(entity[1]) {
                if let Some(new_position) = positions_clone.get(&entity[0]) {
                    *position = new_position.clone();
                }
            };
        });

        if game_end.is_empty() {
            // <-- if verificando se houve um evento de fimd e jogo

            let _ = positions.get_mut(id).map(|(_, _segment, mut pos)| {
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
                    || pos.x as u16 >= GRID_WIDTH // <-- Nova verificação
                    || pos.y as u16 >= GRID_HEIGHT
                {
                    game_end_writer.send(GameEndEvent::GameOver); // <-- publicar evento
                }

                if positions_clone.iter()
                    .filter(|(k, _)| k != &&id)
                    .map(|(_, v)| v)
                    .any(|segment_position| &*pos == segment_position)
                {
                    game_end_writer.send(GameEndEvent::GameOver);
                }

            });
        }
    }
    *last_tail_position = LastTailPosition(Some(
        positions_clone
            .get(segments.last().unwrap())
            .unwrap()
            .clone(),
    ));
}

pub fn eating_system(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<Head>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn growth_system(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<Segments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.read().next().is_some() {
        segments.push(spawn_segment_system(
            commands,
            last_tail_position.0.clone().unwrap(),
        ));
    }
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
        assert_eq!(query.iter(&app.world).count(), 1);
    }

    #[test]
    fn snake_starts_moviment_up() {
        // <-- novo teste
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
        let default_position = Position { x: 5, y: 6 };

        // Adicionando sistemas
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_systems(Startup, spawn_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, movement_input_system.before(movement_system));

        // Adicionando inputs de `KeyCode`s
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::W);
        app.insert_resource(input);

        // Executando sistemas pelo menos uma vez
        app.update();

        //Assert
        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(head, position)| {
            assert_eq!(&default_position, position);
            assert_eq!(head.direction, Direction::Up); // <-- novo assert
        })
    }
    #[test]
    fn snake_head_moves_up_and_right() {
        // Setup
        let mut app = App::new();
        let up_position = Position { x: 5, y: 6 };

        // Adiciona systemas
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_systems(Startup, spawn_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, movement_input_system.before(movement_system));

        // Testa movimento para cima
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::W);
        app.insert_resource(input);
        app.update();

        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(head, position)| {
            assert_eq!(position, &up_position);
            assert_eq!(head.direction, Direction::Up); // <- Novo assert
        });

        let up_right_position = Position { x: 6, y: 6 };

        // Testa movimento para direita
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::D);
        app.insert_resource(input);
        app.update();

        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(head, position)| {
            assert_eq!(&up_right_position, position);
            assert_eq!(head.direction, Direction::Right); // <- Novo assert
        })
    }
    #[test]
    fn snake_head_moves_down_and_left() {
        // Setup
        let mut app = App::new();
        let down_left_position = Position { x: 4, y: 6 };

        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_systems(Startup, spawn_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, movement_input_system.before(movement_system));

        // Movimenta para baixo
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::S);
        app.insert_resource(input);
        app.update();

        // Movimenta para esquerda
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::A);
        app.insert_resource(input);
        app.update();

        // Assert
        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(head, position)| {
            assert_eq!(&down_left_position, position);
            assert_eq!(head.direction, Direction::Left); // <-- Novo Assert
        })
    }
    #[test]
    fn snake_cannot_start_moving_down() {
        // Setup
        let mut app = App::new();
        let down_left_position = Position { x: 5, y: 6 };

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>() // <--
            .add_systems(Startup, spawn_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, movement_input_system.before(movement_system));

        // Move down
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::S);
        app.insert_resource(input);
        app.update();

        // Assert
        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(_head, position)| {
            assert_eq!(&down_left_position, position);
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
        assert_eq!(query.iter(&app.world).count(), 2);
    }

    #[test]
    fn snake_segment_has_followed_head() {
        // Setup
        let mut app = App::new();
        let new_position_head_right = Position { x: 6, y: 5 };
        let new_position_segment_right = Position { x: 5, y: 5 };

        // Adiciona os systemas
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_systems(Startup, spawn_system)
            .add_systems(Update, movement_system)
            .add_systems(Update, movement_input_system.before(movement_system));

        // adiciona resource apertando a tecla D, movimento para direita
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::D);
        app.insert_resource(input);

        // executa sistemas
        app.update();

        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(head, position)| {
            // garante que nova posição da cabeça é esperada:
            assert_eq!(&new_position_head_right, position);
            // garante que nova direção é para direita:
            assert_eq!(head.direction, Direction::Right);
        });

        let mut query = app.world.query::<(&Segment, &Position, Without<Head>)>();
        query.iter(&app.world).for_each(|(_segment, position, _)| {
            // garante que nova posição do segmento é esperada:
            assert_eq!(&new_position_segment_right, position);
        });

        // NOVAS POSIÇÕES ESPERADAS
        let new_position_head_up = Position { x: 6, y: 6 }; // <--
        let new_position_segment_up = Position { x: 6, y: 5 }; // <--

        // adiciona resource apertando a tecla W, movimento para cima
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::W); // <--
        app.insert_resource(input);

        // executa sistemas de novo
        app.update();

        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(head, position)| {
            // garante que nova posição da cabeça é esperada:
            assert_eq!(&new_position_head_up, position);
            // garante que nova direção da cabeça é esperada:
            assert_eq!(head.direction, Direction::Up);
        });

        let mut query = app.world.query::<(&Segment, &Position, Without<Head>)>();
        query.iter(&app.world).for_each(|(_segment, position, _)| {
            // garante que nova posição do segmento é esperada:
            assert_eq!(&new_position_segment_up, position);
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
        assert_eq!(query.iter(&app.world).count(), 2);
        let mut query = app.world.query::<(&Food, &Position)>();
        assert_eq!(query.iter(&app.world).count(), 1);

        // update de execução
        app.update();

        let mut query = app.world.query::<(&Segment, &Position)>();
        assert_eq!(query.iter(&app.world).count(), 3);
    }
}
