pub mod player_event {

    #[derive(Debug)]
    pub enum PlayerEvent {
        AdvanceFrame { frame_count: u32 },
        Land,
    }
}
