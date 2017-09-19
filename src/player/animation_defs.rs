use rendering::animation_seq::*;

pub struct PlayerAnimations;

impl PlayerAnimations {
    pub fn idle() -> AnimationSequence {
        to_seq(Animation::forever(Animation::play(0, 9)))
    }

    pub fn attack() -> AnimationSequence {
        to_seq(Animation::play(0, 9))
    }

    pub fn jump() -> AnimationSequence {
        to_seq(Animation::seq(vec![
            Animation::play(0, 4),
            PlayerAnimations::drop().animation
        ]))
    }

    pub fn drop() -> AnimationSequence {
        to_seq(Animation::forever(Animation::repeat(5, Animation::play(6, 6))))
    }

    pub fn run() -> AnimationSequence {
        to_seq(Animation::forever(Animation::play(0, 9)))
    }

    pub fn slide() -> AnimationSequence {
        to_seq(Animation::play(0, 9))
    }
}

fn to_seq(a: Animation) -> AnimationSequence {
    AnimationSequence::new(a)
}
