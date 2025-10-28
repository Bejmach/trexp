use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, symbols::{border, line}, text::{Line, Span}, widgets::{Block, List, ListItem, Padding, Paragraph}, Frame};

use crate::app::{App, AppComponent, AppEdit, AppState};

pub fn ui(frame: &mut Frame, app: &mut App){
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(70),
            Constraint::Percentage(30),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);
    let task_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(inner_chunks[1]);

    let hint_binds = if app.error_message != "" || app.result_message != ""{
        vec![("Enter", "Accept")]
    }else{
        match app.state{
            AppState::Main => vec![
                ("q", "Quit"),
                ("Enter", "Select"),
                ("Arrows", "Move"),
            ],
            AppState::Categories => vec![
                ("q", "Quit"),
                ("Enter", "Select"),
                ("up/down", "Move"),
                ("pgUp/pgDown", "Move category"),
                ("n", "New"),
                ("d", "Delete"),
                ("e", "Edit"),
            ],
            AppState::CreateCategory | AppState::EditCategory => vec![
                ("Enter", "Accept"),
                ("Esc", "Cancel"),
            ],
            AppState::Tasks | AppState::Milestones => vec![
                ("q", "Quit"),
                ("Enter", "Select"),
                ("up/down", "Move"),
                ("pgUp/pgDown", "Move task"),
                ("n", "New"),
                ("e", "Edit"),
                ("d", "Delete"),
                ("c", "Complete"),
            ],
            AppState::CreateTask | AppState::CreateMilestone => vec![
                ("Enter", "Accept"),
                ("Esc", "Cancel"),
                ("left/right", "Change edit box"),
            ],
            _ => vec![
                ("q", "Quit"),
            ],
        }
    };

    frame.render_widget(Paragraph::new("Program name").centered(), chunks[0]);
    render_categories(app, frame, inner_chunks[0]);
    render_tasks(app, frame, task_chunks[0]);
    render_milestones(app, frame, task_chunks[1]);
    render_timers(app, frame, chunks[2]);
    render_help(app, frame, chunks[3], hint_binds);

    match app.state{
        AppState::CreateCategory | AppState::EditCategory => {
            render_new_category(app, frame, 60, 25, inner_chunks[0]);
        },
        AppState::CreateTask => {
            render_new_task(app, frame, 60, 25, task_chunks[0]);
        },
        AppState::CreateMilestone => {
            render_new_milestone(app, frame, 60, 25, task_chunks[1]);
        }
        _ => {}
    }

    if app.result_message != "" {
        render_result(app, frame, 40, 20, frame.area());
    }
    if app.error_message != "" {
        render_error(app, frame, 40, 20, frame.area());
    }
}

fn render_categories(app: &mut App, frame: &mut Frame, area: Rect){
    let mut items: Vec<ListItem> = Vec::new();
    let size: u16 = area.width;
    for (i, category) in app.data.categories.iter().enumerate(){
        let bar_size = size as usize - 20 - format!("{}", category.lvl).chars().count() - 12;
        let category_bars: u32 = category.exp * bar_size as u32 / category.exp_to_next_lvl;
        let bar: String = format!("|{:_<1$}|", format!("{:X<1$}", "", category_bars as usize), bar_size);

        let category_text = format!("{: <20} {} {}", category.name, bar, category.lvl);

        let style = if i == app.cur_category as usize{
            if app.state == AppState::Categories{
                app.theme.selection
            }
            else{
                app.theme.faded_selection
            }
        }else{
            app.theme.passive
        };

        items.push(ListItem::new(Line::from(Span::styled(
            category_text,
            style,
        ))));
    }
    let category_list = List::new(items);

    let style = if app.state == AppState::Categories{
        app.theme.active
    }else if app.get_cur_component() == Some(&AppComponent::Categories){
        app.theme.focus
    }else{
        app.theme.passive
    };

    let block = Block::bordered()
        .title(Line::from(" Categories ".bold()))
        .border_set(border::PLAIN)
        .padding(Padding::new(2, 4, 1, 1))
        .style(style);

    frame.render_widget(category_list.block(block), area);
}

fn render_tasks(app: &mut App, frame: &mut Frame, area: Rect){
    let mut items: Vec<ListItem> = Vec::new();

    if let Some(category) = app.data.get_category(app.cur_category as usize){
        for (i, task) in category.tasks.iter().enumerate(){
            let task_text = format!("{} [+{} XP]", task.name, task.exp_reward);

            let style = if i == app.cur_task as usize {
                if app.state == AppState::Tasks{
                    app.theme.selection
                }
                else{
                    app.theme.faded_selection
                }
            }else{
                app.theme.passive
            };

            items.push(ListItem::new(Line::from(Span::styled(
                task_text,
                style
            ))));
        }
    }

    let task_list = List::new(items);

    let style = if app.state == AppState::Tasks{
        app.theme.active
    }else if app.get_cur_component() == Some(&AppComponent::Tasks){
        app.theme.focus
    }else{
        app.theme.passive
    };

    let block = Block::bordered()
        .title(Line::from(" Tasks ".bold()))
        .border_set(border::PLAIN)
        .padding(Padding::new(2, 4, 1, 1))
        .style(style);

    frame.render_widget(task_list.block(block), area);
}

fn render_milestones(app: &mut App, frame: &mut Frame, area: Rect){
    let mut items: Vec<ListItem> = Vec::new();

    if let Some(category) = app.data.get_category(app.cur_category as usize){
        for (i, milestone) in category.milestones.iter().enumerate(){
            let milestone_text = format!("{} [+{} XP]", milestone.name, milestone.exp_reward);

            let style = if i == app.cur_milestone as usize {
                if app.state == AppState::Milestones{
                    app.theme.selection
                }
                else{
                    app.theme.faded_selection
                }
            }else{
                app.theme.passive
            };

            items.push(ListItem::new(Line::from(Span::styled(
                milestone_text,
                style
            ))));
        }
    }

    let milestone_list = List::new(items);

    let style = if app.state == AppState::Milestones{
        app.theme.active
    }else if app.get_cur_component() == Some(&AppComponent::Milestones){
        app.theme.focus
    }else{
        app.theme.passive
    };

    let block = Block::bordered()
        .title(Line::from(" Milestones ".bold()))
        .border_set(border::PLAIN)
        .padding(Padding::new(2, 4, 1, 1))
        .style(style);

    frame.render_widget(milestone_list.block(block), area);
}

fn render_timers(app: &mut App, frame: &mut Frame, area: Rect){
    let style = if app.state == AppState::Timers{
        app.theme.active
    }else if app.get_cur_component() == Some(&AppComponent::Timers){
        app.theme.focus
    }else{
        app.theme.passive
    };

    let block = Block::bordered()
        .title(Line::from(" Timers ".bold()))
        .border_set(border::PLAIN)
        .padding(Padding::new(2, 4, 1, 1))
        .style(style);

    frame.render_widget(block, area);
}

fn render_new_category(app: &mut App, frame: &mut Frame, width: u16, height: u16, area: Rect){
    let block = Block::bordered()
        .title(Line::from("New Category".bold()))
        .border_set(border::ROUNDED)
        .padding(Padding::new(3, 3, 1, 1));

    let paragraph = Paragraph::new(app.edit_name.clone())
        .centered()
        .block(block)
        .style(app.theme.floating); 
    let area = centered_rect(width, height, area);
    frame.render_widget(paragraph, area);
}

fn render_new_task(app: &mut App, frame: &mut Frame, width: u16, height: u16, area: Rect){
    let block = Block::bordered()
        .title_top(Line::from("New Task".bold()).centered())
        .border_set(border::ROUNDED)
        .padding(Padding::new(3, 3, 1, 1))
        .style(app.theme.floating);

    let area = centered_rect(width, height, area);
    frame.render_widget(block, area);

    let task_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let name_style = if app.cur_edit == AppEdit::Name{
        app.theme.active
    }else{
        app.theme.floating
    };
    let exp_style = if app.cur_edit == AppEdit::Exp{
        app.theme.active
    }else{
        app.theme.floating
    };

    let name_block = Block::bordered()
        .title(Line::from("Name").centered())
        .border_set(border::PLAIN)
        .style(name_style);

    let exp_block = Block::bordered()
        .title(Line::from("Revard").centered())
        .border_set(border::PLAIN)
        .style(exp_style);
    
    let name_paragraph = Paragraph::new(app.edit_name.clone())
        .centered()
        .block(name_block)
        .style(name_style);

    let exp_paragraph = Paragraph::new(app.edit_exp.clone())
        .centered()
        .block(exp_block)
        .style(exp_style);

    frame.render_widget(name_paragraph, task_layout[0]);
    frame.render_widget(exp_paragraph, task_layout[1]);
}

fn render_new_milestone(app: &mut App, frame: &mut Frame, width: u16, height: u16, area: Rect){
    let block = Block::bordered()
        .title_top(Line::from("New Milestone".bold()).centered())
        .border_set(border::ROUNDED)
        .padding(Padding::new(3, 3, 1, 1))
        .style(app.theme.floating);

    let area = centered_rect(width, height, area);
    frame.render_widget(block, area);

    let task_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let name_style = if app.cur_edit == AppEdit::Name{
        app.theme.active
    }else{
        app.theme.floating
    };
    let exp_style = if app.cur_edit == AppEdit::Exp{
        app.theme.active
    }else{
        app.theme.floating
    };

    let name_block = Block::bordered()
        .title(Line::from("Name").centered())
        .border_set(border::PLAIN)
        .style(name_style);

    let exp_block = Block::bordered()
        .title(Line::from("Revard").centered())
        .border_set(border::PLAIN)
        .style(exp_style);
    
    let name_paragraph = Paragraph::new(app.edit_name.clone())
        .centered()
        .block(name_block)
        .style(name_style);

    let exp_paragraph = Paragraph::new(app.edit_exp.clone())
        .centered()
        .block(exp_block)
        .style(exp_style);

    frame.render_widget(name_paragraph, task_layout[0]);
    frame.render_widget(exp_paragraph, task_layout[1]);
}

fn render_result(app: &mut App, frame: &mut Frame, width: u16, height: u16, area: Rect){
    let block = Block::bordered()
        .border_set(border::ROUNDED)
        .padding(Padding::new(3, 3, 1, 1));

    let paragraph = Paragraph::new(app.result_message.clone())
        .centered()
        .block(block)
        .style(app.theme.floating);

    let area = centered_rect(width, height, area);
    frame.render_widget(paragraph, area);
}

fn render_error(app: &mut App, frame: &mut Frame, width: u16, height: u16, area: Rect){
    let block = Block::bordered()
        .border_set(border::ROUNDED)
        .padding(Padding::new(3, 3, 1, 1));

    let paragraph = Paragraph::new(app.error_message.clone())
        .centered()
        .block(block)
        .style(app.theme.error);

    let area = centered_rect(width, height, area);
    frame.render_widget(paragraph, area);
}

fn render_help(app: &mut App, frame: &mut Frame, area: Rect, keybinds: Vec<(&str, &str)>){
    let mut span_vec: Vec<Span> = Vec::new();
    let mut line_vec: Vec<Line> = Vec::new();
    
    let key_style = app.theme.help_key;
    let command_style = app.theme.help_text;

    let max_width = frame.area().width - 10;
    let mut counter: u16 = 0;

    for (key, command) in keybinds {
        counter += key.chars().count() as u16 + command.chars().count() as u16 + 4;

        if counter >= max_width{
            line_vec.push(Line::from(span_vec.clone()));
            span_vec = Vec::new();
            counter = 0;
        }

        span_vec.append( &mut vec![
            Span::styled(key, key_style),
            Span::raw(" "),
            Span::styled(command, command_style),
            Span::raw("   ")
        ]);
    }

    if span_vec.len() != 0 {
        line_vec.push(Line::from(span_vec.clone()));
    }

    let paragraph = Paragraph::new(line_vec);

    frame.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
