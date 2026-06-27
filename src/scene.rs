

pub struct Choice {
    pub text: String,
    pub next_scene_id: String,
}

pub struct Scene {
    pub title: String,
    pub story: String,
    pub choices: Vec<Choice>,
}