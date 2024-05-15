use tracing::info;

#[derive(Debug, Clone)]
pub enum Instruct {
    // 移动
    Move(i32, i32),
    Mouse(String, usize),
    Sleep(usize),
    Loop(usize, Vec<Instruct>),
}

impl Instruct {
    pub fn execute(&self) {
        match self {
            Instruct::Loop(count, insrants) => {
                for _ in 0..*count {
                    for ins in insrants {
                        ins.execute();
                    }
                }
            }
            Instruct::Move(x, y) => {
                info!("移动到位置: ({}, {})", x, y);
            }
            Instruct::Mouse(button, count) => {
                info!("鼠标 {} 按钮点击 {} 次", button, count);
            }
            Instruct::Sleep(duration) => {
                info!("等待 {} 毫秒", duration);
                // std::thread::sleep(Duration::from_millis(*duration as u64));
            }
        }
    }
}
