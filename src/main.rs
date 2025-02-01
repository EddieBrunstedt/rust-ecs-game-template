use raylib::prelude::*;

struct Player {
    position: Vector2,
    speed: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Moving Dot with Camera")
        .build();

    // Create player at center of screen
    let mut player = Player {
        position: Vector2::new(0.0, 0.0),
        speed: 0.1,
    };

    // Generate random background dots
    let background_dots: Vec<Vector2> = (0..50)
        .map(|_| {
            Vector2::new(
                rand::random::<f32>() * 1000.0 - 500.0,
                rand::random::<f32>() * 1000.0 - 500.0,
            )
        })
        .collect();

    // Create camera
    let mut camera = Camera2D {
        target: player.position,
        offset: Vector2::new(320.0, 240.0), // Half the screen size
        rotation: 0.0,
        zoom: 1.0,
    };

    while !rl.window_should_close() {
        // Update player position based on input
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            player.position.x += player.speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            player.position.x -= player.speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            player.position.y += player.speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            player.position.y -= player.speed;
        }

        // Update camera to follow player
        camera.target = player.position;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Begin 2D camera mode
        let mut camera_draw = d.begin_mode2D(camera);

        // Draw background dots
        for dot_pos in &background_dots {
            camera_draw.draw_circle(dot_pos.x as i32, dot_pos.y as i32, 4.0, Color::GRAY);
        }

        // Draw player dot (will appear centered due to camera)
        camera_draw.draw_circle(
            player.position.x as i32,
            player.position.y as i32,
            8.0,
            Color::RED,
        );
    }
}
