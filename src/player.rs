pub mod player_state {
    use crate::events::player_event::PlayerEvent;
    use statig::{prelude::*, state_machine};

    #[derive(Debug, Default)]
    pub struct PlayerStateMachine {
        frame: u16,
    }

    #[state_machine(
        initial = "State::falling()", state(derive(Debug)),
        // Derive the Debug trait on the `State` enum.
        state(derive(Debug)),
        // Derive the Debug trait on the `Superstate` enum.
        superstate(derive(Debug)),
        // Set the `on_transition` callback.
        on_transition = "Self::on_transition",
        // Set the `on_dispatch` callback.
        on_dispatch = "Self::on_dispatch",
    )]
    impl PlayerStateMachine {
        #[state]
        fn falling(event: &PlayerEvent) -> Response<State> {
            match event {
                _ => Super,
            }
        }

        #[state]
        fn landing(event: &PlayerEvent) -> Response<State> {
            match event {
                _ => Super,
            }
        }

        #[state]
        fn idle(event: &PlayerEvent) -> Response<State> {
            match event {
                _ => Super,
            }
        }
    }

    impl PlayerStateMachine {
        // The `on_transition` callback that will be called after every transition.
        fn on_transition(&mut self, source: &State, target: &State) {
            println!("transitioned from `{source:?}` to `{target:?}`");
        }

        fn on_dispatch(&mut self, state: StateOrSuperstate<Self>, event: &PlayerEvent) {
            println!("dispatching `{event:?}` to `{state:?}`");
        }
    }
}
