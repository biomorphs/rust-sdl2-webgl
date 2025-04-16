#[derive(Default)]
pub struct MouseState
{
    pub position_x: i32,
    pub position_y: i32,
    pub left_btn_down: bool,
    pub middle_btn_down: bool,
    pub right_btn_down: bool
}

#[derive(Default)]
pub struct InputState
{
    pub mouse_state: MouseState
}