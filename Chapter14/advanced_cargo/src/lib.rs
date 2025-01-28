// 크레이트 전체를 설명할때에는 //! 를 사용한다.
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
// 이렇게 re-export를 사용하면 사용자가 이 크레이트를 사용할때에는
// use art::PrimaryColor; 와 같이 사용할 수 있다.
// 만약 사용하지 않았다면, 사용자 입장에서는 use art::kinds::PrimaryColor; 와 같이 사용해야 한다.
// 즉, 사용자 입장에서는 사용성에 혼동을 줄 수 있으므로, re-export를 사용하는 것이 좋다.

pub mod kinds {
    // 특정 함수에 대한 Doc 주석을 달때에는 /// 를 사용한다.
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --생략--
    }
}