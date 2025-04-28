use rmk::action::KeyAction;
use rmk::{a, k, mo, tg, shifted};
pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 8;
#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        [
            [k!(Grave), k!(Comma), k!(Dot), k!(P), k!(Y), k!(L), k!(R), k!(C), k!(G), k!(F)],
            [k!(A), k!(O), k!(E), k!(U), k!(I), k!(S), k!(N), k!(T), k!(H), k!(D)],
            [k!(Slash), k!(Q), k!(J), k!(K), k!(X), k!(Z), k!(V), k!(W), k!(M), k!(B)],
            [a!(Transparent), a!(Transparent), a!(Transparent), k!(LShift), mo!(1), a!(Transparent), a!(Transparent), a!(Transparent), k!(Space), mo!(2)],
        ],
        [
            [k!(BrightnessDown), k!(BrightnessUp), k!(Tab), k!(AudioVolDown), k!(AudioVolUp), k!(Delete), k!(Enter), k!(Up), k!(End), k!(Home)],
            [k!(LShift), k!(LCtrl), k!(LAlt), k!(LGui), k!(AudioMute), k!(Backspace), k!(Right), k!(Down), k!(Left), k!(PageUp)],
            [k!(PrintScreen), k!(Z), k!(X), k!(C), k!(V), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), k!(PageDown)],
            [a!(Transparent), a!(Transparent), a!(Transparent), tg!(6), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), mo!(5), mo!(2)]
        ],
        [
            [shifted!(Kc2), k!(RightBracket), shifted!(RightBracket), shifted!(Kc9), shifted!(Kc4), shifted!(Dot), k!(NonusHash), shifted!(NonusHash), shifted!(Kc0), shifted!(Comma)],
            [k!(Escape), shifted!(Kc8), k!(Minus), k!(KpEqual), shifted!(Equal), k!(RShift), k!(RCtrl), k!(LAlt), k!(LGui), shifted!(Kc7)],
            [shifted!(NonusBackslash), shifted!(Kc3), shifted!(Minus), shifted!(Kc1), shifted!(Kc5), shifted!(Quote), shifted!(International1), k!(Quote), k!(International1), k!(NonusBackslash)],
            [a!(Transparent), a!(Transparent), a!(Transparent), mo!(4), mo!(1), a!(Transparent), a!(Transparent), a!(Transparent), k!(LeftBracket), a!(Transparent)]
        ],
        [
            [k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), k!(Kc0), k!(Kc9), k!(Kc8), k!(Kc7), k!(Kc6)],
            [k!(LShift), k!(LCtrl), k!(LAlt), k!(LGui), k!(F11), k!(RShift), k!(RCtrl), k!(LAlt), k!(LGui), k!(F12)],
            [k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F10), k!(F9), k!(F8), k!(F7), k!(F6)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)]
        ],
        [
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), k!(Semicolon), a!(Transparent), a!(Transparent)],
            [k!(Macro0), k!(Macro1), k!(Macro2), k!(Macro3), k!(Macro4), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)]
        ],
        [
            [a!(Transparent), k!(MouseBtn2), k!(MouseUp), k!(MouseBtn1), k!(MouseWheelUp), k!(User11), k!(User10), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), k!(MouseLeft), k!(MouseDown), k!(MouseRight), k!(MouseWheelDown), k!(User4), k!(User3), k!(User2), k!(User1), k!(User0)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)]
        ],
        [
            [k!(Tab), k!(Q), k!(W), k!(E), k!(R), a!(Transparent), a!(Transparent), k!(Up), a!(Transparent), a!(Transparent)],
            [k!(LShift), k!(A), k!(S), k!(D), k!(F), a!(Transparent), k!(Right), k!(Down), k!(Left), a!(Transparent)],
            [k!(LCtrl), k!(Z), k!(X), k!(C), k!(V), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), k!(Space), mo!(7), a!(Transparent), a!(Transparent), a!(Transparent), k!(Space), a!(Transparent)]
        ],
        [
            [k!(Kc5), k!(Kc4), a!(Transparent), k!(Kc1), k!(Kc2), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [k!(Escape), a!(Transparent), a!(Transparent), k!(Kc3), k!(Enter), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [k!(Grave), k!(B), k!(T), k!(P), k!(Y), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), tg!(6), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)]
        ],
    ]
}
