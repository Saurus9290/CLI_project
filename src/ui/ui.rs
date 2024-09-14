pub mod ui {
    use crate::models::{Epic, Story, Status};

    pub struct Prompts {
        pub create_epic: Box<dyn Fn() -> Epic>,
        pub update_status: Box<dyn Fn() -> Option<Status>>,
        pub delete_epic: Box<dyn Fn() -> bool>,
        pub create_story: Box<dyn Fn() -> Story>,
        pub delete_story: Box<dyn Fn() -> bool>,
    }

    impl Prompts {
        pub fn new() -> Self {
            Self {
                create_epic: Box::new(|| Epic::new("".to_owned(), "".to_owned())),
                update_status: Box::new(|| None),
                delete_epic: Box::new(|| false),
                create_story: Box::new(|| Story::new("".to_owned(), "".to_owned())),
                delete_story: Box::new(|| false),
            }
        }
    }
    
    pub trait Page {
        fn as_any(&self) -> &dyn std::any::Any;
    }

    pub struct HomePage { pub db: std::rc::Rc<crate::db::JiraDatabase> }
    pub struct EpicDetail { pub epic_id: u64, pub db: std::rc::Rc<crate::db::JiraDatabase> }
    pub struct StoryDetail { pub epic_id: u64, pub story_id: u64, pub db: std::rc::Rc<crate::db::JiraDatabase> }

    // Implement Page trait for each of the pages (HomePage, EpicDetail, StoryDetail)
    impl Page for HomePage {
        fn as_any(&self) -> &dyn std::any::Any { self }
    }
    
    impl Page for EpicDetail {
        fn as_any(&self) -> &dyn std::any::Any { self }
    }
    
    impl Page for StoryDetail {
        fn as_any(&self) -> &dyn std::any::Any { self }
    }
}
