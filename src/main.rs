use std::{
    io,
    time::{Instant, Duration}
};

use tui::{
    backend::{Backend, CrosstermBackend},
    terminal::{Terminal, Frame},
    layout::{Layout, Direction, Constraint, Alignment},
    widgets::{Paragraph, Table, Cell, Row, Block, Borders, BorderType,
        canvas::{Canvas, Rectangle, Points, Context}
    },
    style::{Style, Color},
    text::Span
};

use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
    execute
};

mod snake;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let frame_duration = Duration::from_millis(30);
    let mut width = terminal.size()?.width * 2;
    let mut height = (terminal.size()?.height - 6) * 4;
    let mut snake = snake::Snake::new(width, height);

    //Main game loop

    loop {
        //Game
        match snake.game_mode {
            snake::GameMode::Play => snake.forward(&width, &height),
            _ => ()
        }

        //Rendering
        terminal.draw(|frame| ui(frame, &snake, &mut width, &mut height))?;

        //Input handling
        let now = Instant::now();

        while now.elapsed() < frame_duration {
            if event::poll(frame_duration - now.elapsed())? {
                if let Event::Key(key) = event::read()? {
                    match key.code {

                        KeyCode::Char('h') => { match snake.direction {
                            snake::Direction::Right => (),
                            snake::Direction::Left => (),
                            _ => {
                                snake.direction = snake::Direction::Left;
                                snake.score = if snake.score > 0 { snake.score - 1 } else { 0 };
                            }
                        }},
                        KeyCode::Char('j') => { match snake.direction {
                            snake::Direction::Up => (),
                            snake::Direction::Down => (),
                            _ => {
                                snake.direction = snake::Direction::Down;
                                snake.score = if snake.score > 0 { snake.score - 1 } else { 0 };
                            }
                        }},
                        KeyCode::Char('k') => { match snake.direction {
                            snake::Direction::Down => (),
                            snake::Direction::Up => (),
                            _ => {
                                snake.direction = snake::Direction::Up;
                                snake.score = if snake.score > 0 { snake.score - 1 } else { 0 };
                            }
                        }},
                        KeyCode::Char('l') => { match snake.direction {
                            snake::Direction::Left => (),
                            snake::Direction::Right => (),
                            _ => {
                                snake.direction = snake::Direction::Right;
                                snake.score = if snake.score > 0 { snake.score - 1 } else { 0 };
                            }
                        }},

                        KeyCode::Char(' ') => snake.game_mode = match snake.game_mode {
                            snake::GameMode::Play => snake::GameMode::Pause,
                            snake::GameMode::Pause => snake::GameMode::Play,
                            snake::GameMode::Lost => snake::GameMode::Lost
                        },
                        KeyCode::Char('r') => {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                            main()?;
                            return Ok(());
                        },
                        KeyCode::Char('q') => {
                            //Quit
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                            return Ok(());
                        },

                        _ => ()
                    }
                }
            }
        }
    }
}

fn draw_game(ctx: &mut Context, snake: &snake::Snake, width: &u16, height: &u16) {
    //Draw the snake
    let mut snake_coords = snake.coords.clone();
    snake_coords.push((snake_coords[1].0 - 1.0, snake_coords[1].1));
    snake_coords.push((snake_coords[1].0 + 1.0, snake_coords[1].1));
    snake_coords.push((snake_coords[1].0, snake_coords[1].1 - 1.0));
    snake_coords.push((snake_coords[1].0, snake_coords[1].1 + 1.0));

    ctx.draw(&Points {
        coords: snake_coords.as_slice(),
        color: Color::Green
    });

    //Draw the apple
    ctx.draw(&Points {
        coords: &[
            snake.apple_coords,
            ((snake.apple_coords.0 - 1.0), snake.apple_coords.1),
            ((snake.apple_coords.0 + 1.0), snake.apple_coords.1),
            (snake.apple_coords.0, (snake.apple_coords.1 - 1.0)),
            (snake.apple_coords.0, (snake.apple_coords.1 + 1.0))
        ],
        color: Color::LightRed
    });

    match snake.game_mode {
        snake::GameMode::Pause => {
            ctx.print(
                (width / 2 - 4) as f64,
                (height / 2) as f64,
                Span::styled("PAUSE", Style::default().fg(Color::LightRed))
            );
        },
        snake::GameMode::Lost => {
            ctx.draw(&Rectangle {
                x: (width / 2 - 12) as f64,
                y: (height / 2 - 4) as f64,
                width: 24.0,
                height: 8.0,
                color: Color::Red
            });
            ctx.print(
                (width / 2 - 6) as f64,
                (height / 2) as f64,
                Span::styled("YOU LOST", Style::default().fg(Color::LightRed))
            );
        },
        _ => ()
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, snake: &snake::Snake, width: &mut u16, height: &mut u16) {
    let size = frame.size();

    //Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3)
            ]
            .as_ref()
        )
        .split(size);

    *width = chunks[1].width * 2;
    *height = chunks[1].height * 4;

    //Title
    let title_bar = Paragraph::new("- Cobrust -")
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
        );

    frame.render_widget(title_bar, chunks[0]);

    //Main game window
    let arena = Canvas::default()
        .block(
            Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
        )
        .paint(|ctx| draw_game(ctx, &snake, width, height))
        .x_bounds([0.0, *width as f64])
        .y_bounds([0.0, *height as f64]);

    frame.render_widget(arena, chunks[1]);

    //Information footer
    let info_bar = Table::new(vec![
        Row::new(vec![
            Cell::from(match snake.game_mode {
                snake::GameMode::Pause => "      \u{2190}: [h]    \u{2193}: [j]    \u{2191}: [k]    \u{2192}: [l]    play: [space]    restart: [r]    quit: [q]",
                _ => "      \u{2190}: [h]    \u{2193}: [j]    \u{2191}: [k]    \u{2192}: [l]    pause: [space]    restart: [r]    quit: [q]"
            }),
            Cell::from(
                Span::styled(format!("Score : {}", snake.score), Style::default().fg(Color::LightYellow))
            )
        ])
    ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Keys")
                .border_type(BorderType::Rounded)
        )
        .widths(&[Constraint::Percentage(85), Constraint::Percentage(15)]);

    frame.render_widget(info_bar, chunks[2]);
}
