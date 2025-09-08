use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::thread;
use rand::Rng;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Color, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

// Screen Resolution
const SCREEN_WIDTH: u16 = 42;
const SCREEN_HEIGHT: u16 = 32;

fn main()
{
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();

    let mut rng = rand::rng();

    // Initialize snake
    let mut snake: Vec<(u16, u16)> = vec![
        (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4),
        (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4 - 1),
        (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4 - 2),
        (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4 - 3),
    ];
    
    // Initialize food and game state
    let mut food = (rng.random_range(2..SCREEN_HEIGHT - 1), rng.random_range(1..SCREEN_WIDTH - 1));
    let mut score = 0;
    let mut direction = KeyCode::Right;
    let mut update_time = Instant::now();
    let update_interval = Duration::from_millis(100);

    // Game loop
    'game: loop
    {
        // Process input
        while event::poll(Duration::from_millis(0)).unwrap()
        {
            if let Event::Key(key_event) = event::read().unwrap()
            {
                direction = match key_event.code {
                    KeyCode::Left | KeyCode::Char('a') if direction != KeyCode::Right => KeyCode::Left,
                    KeyCode::Right | KeyCode::Char('d') if direction != KeyCode::Left => KeyCode::Right,
                    KeyCode::Up | KeyCode::Char('w') if direction != KeyCode::Down => KeyCode::Up,
                    KeyCode::Down | KeyCode::Char('s') if direction != KeyCode::Up => KeyCode::Down,
                    KeyCode::Char('q') => break 'game, // Quit with 'q'
                    _ => direction,
                };
            }
        }

        if update_time.elapsed() >= update_interval
        {
            let snake_head = snake[0];
            let new_head = match direction {
                KeyCode::Left => (snake_head.0, snake_head.1 - 1),
                KeyCode::Right => (snake_head.0, snake_head.1 + 1),
                KeyCode::Up => (snake_head.0 - 1, snake_head.1),
                KeyCode::Down => (snake_head.0 + 1, snake_head.1),
                _ => snake_head,
            };

            // Check for collision with border or self by checking the new head against the snake's body
            let self_collision = snake.iter().any(|&pos| pos == new_head);

            if new_head.0 <= 1 || new_head.0 >= SCREEN_HEIGHT || new_head.1 == 0 || new_head.1 >= SCREEN_WIDTH - 1 || self_collision
            {
                // Game over screen
                queue!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
                
                let game_over_msg = vec![
                    ("Game Over!".to_string(), Color::Red),
                    (format!("Final Score: {}", score), Color::Reset),
                    ("Press 'r' to restart or 'q' to quit".to_string(), Color::Blue),
                ];
                
                for (i, (line, color)) in game_over_msg.iter().enumerate()
                {
                    let x = SCREEN_WIDTH / 2 - (line.len() as u16 / 2);
                    queue!(stdout, cursor::MoveTo(x, SCREEN_HEIGHT / 2 + i as u16), SetForegroundColor(*color)).unwrap();
                    print!("{}", line);
                    queue!(stdout, SetForegroundColor(Color::Reset)).unwrap();
                }
                stdout.flush().unwrap();

                // Wait for 'r' or 'q'
                loop
                {
                    if event::poll(Duration::from_millis(100)).unwrap()
                    {
                        if let Event::Key(key_event) = event::read().unwrap()
                        {
                            match key_event.code
                            {
                                KeyCode::Char('r') => {
                                    // Restart game
                                    snake = vec![
                                        (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4),
                                        (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4 - 1),
                                        (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4 - 2),
                                        (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4 - 3),
                                    ];
                                    
                                    food = (rng.random_range(2..SCREEN_HEIGHT - 1), rng.random_range(1..SCREEN_WIDTH - 1));
                                    score = 0;
                                    direction = KeyCode::Right;
                                    break;
                                }

                                KeyCode::Char('q') => break 'game,

                                _ => continue,
                            }
                        }
                    }
                }
                continue; // Restart the game loop
            }

            // Update snake position
            snake.insert(0, new_head);

            if new_head == food
            {
                score += 10;

                // Find total score and subtract the snake length
                let total_score = (SCREEN_HEIGHT as u16 - 2) * (SCREEN_WIDTH as u16 - 2);
                let snake_len: u16 = 4;
                let combine = total_score - snake_len;

                // Check for win condition
                if (score / 10) as u16 == combine
                {
                    // Win screen
                    queue!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
                    let congrats_msg = vec![
                        ("Congratulations! You Win!".to_string(), Color::Green),
                        (format!("Final Score: {}", score), Color::Reset),
                        ("Press 'r' to restart or 'q' to quit".to_string(), Color::Blue),
                    ];
                    
                    for (i, (line, color)) in congrats_msg.iter().enumerate()
                    {
                        let x = SCREEN_WIDTH / 2 - (line.len() as u16 / 2);
                        queue!(stdout, cursor::MoveTo(x, SCREEN_HEIGHT / 2 + i as u16), SetForegroundColor(*color)).unwrap();
                        print!("{}", line);
                        queue!(stdout, SetForegroundColor(Color::Reset)).unwrap();
                    }
                    stdout.flush().unwrap();

                    // Wait for 'r' or 'q'
                    loop
                    {
                        if event::poll(Duration::from_millis(100)).unwrap()
                        {
                            if let Event::Key(key_event) = event::read().unwrap()
                            {
                                match key_event.code
                                {
                                    KeyCode::Char('r') => {
                                        // Restart game
                                        snake = vec![
                                            (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4),
                                            (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4 - 1),
                                            (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4 - 2),
                                            (SCREEN_HEIGHT / 2 + 1, SCREEN_WIDTH / 4 - 3),
                                        ];

                                        food = (rng.random_range(2..SCREEN_HEIGHT - 1), rng.random_range(1..SCREEN_WIDTH - 1));
                                        score = 0;
                                        direction = KeyCode::Right;
                                        break;
                                    }

                                    KeyCode::Char('q') => break 'game,

                                    _ => continue,
                                }
                            }
                        }
                    }
                    continue; // Restart the game loop
                }
                food = loop {
                    let new_food = (rng.random_range(2..SCREEN_HEIGHT - 1), rng.random_range(1..SCREEN_WIDTH - 1));
                    
                    if !snake.iter().any(|&pos| pos == new_food)
                    {
                        break new_food;
                    }
                };
            }
            else
            {
                snake.pop();
            }

            // Render game
            queue!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

            // Draw score at top
            let score_text = format!("Score: {}", score);
            let score_x = SCREEN_WIDTH / 2 - (score_text.len() as u16 / 2);
            queue!(stdout, cursor::MoveTo(score_x, 0)).unwrap();
            print!("{}", score_text);

            // Draw border (yellow)
            for x in 0..SCREEN_WIDTH
            {
                queue!(stdout, cursor::MoveTo(x, 1), SetForegroundColor(Color::Yellow)).unwrap();
                print!("#");
                queue!(stdout, SetForegroundColor(Color::Reset)).unwrap();
                queue!(stdout, cursor::MoveTo(x, SCREEN_HEIGHT), SetForegroundColor(Color::Yellow)).unwrap();
                print!("#");
                queue!(stdout, SetForegroundColor(Color::Reset)).unwrap();
            }
            
            for y in 1..=SCREEN_HEIGHT
            {
                queue!(stdout, cursor::MoveTo(0, y), SetForegroundColor(Color::Yellow)).unwrap();
                print!("#");
                queue!(stdout, SetForegroundColor(Color::Reset)).unwrap();
                queue!(stdout, cursor::MoveTo(SCREEN_WIDTH - 1, y), SetForegroundColor(Color::Yellow)).unwrap();
                print!("#");
                queue!(stdout, SetForegroundColor(Color::Reset)).unwrap();
            }

            // Draw snake (green)
            for &(y, x) in &snake
            {
                queue!(stdout, cursor::MoveTo(x, y), SetForegroundColor(Color::Green)).unwrap();
                print!("@");
                queue!(stdout, SetForegroundColor(Color::Reset)).unwrap();
            }

            // Draw food (red)
            queue!(stdout, cursor::MoveTo(food.1, food.0), SetForegroundColor(Color::Red)).unwrap();
            print!("■");
            queue!(stdout, SetForegroundColor(Color::Reset)).unwrap();

            stdout.flush().unwrap();
            update_time = Instant::now();
        }

        thread::sleep(Duration::from_millis(10));
    }

    // Exit game
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
}

