/// Reference: symbols.js, types.js

use serde::Serialize;
use crate::katex::types::Mode;
use crate::katex::node::*;

pub struct Symbol {
    pub mode: Mode,
    pub font: Font,
    pub group: Group,
    pub name: char
}

pub enum Font {
    Main,
    Ams,
}

pub enum Group {
    Atom(AtomGroup),
    NonAtom(NonAtomGroup),
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AtomGroup {
    Bin,
    Close,
    Inner,
    Open,
    Punct,
    Rel,
}

pub enum NonAtomGroup {
    AccentToken,
    MathOrd,
    OpToken,
    Spacing,
    TextOrd,
}

impl<'a> Symbol {
    pub fn new(mode: Mode, font: Font, group: Group, name: char) -> Self {
        Self {
            mode: mode,
            font: font,
            group: group,
            name: name
        }
    }

    pub fn create_node(self: Self) -> Node {
        match self.group {
            Group::Atom(group) => {
                AtomBuilder::default()
                    .family(group)
                    .mode(self.mode)
                    .text(self.name.to_string())
                    .build().unwrap().into_node()
            },
            Group::NonAtom(group) => {
                match group {
                    NonAtomGroup::MathOrd => {
                        MathOrdBuilder::default()
                            .mode(self.mode)
                            .text(self.name.to_string())
                            .build().unwrap().into_node()
                    },
                    NonAtomGroup::OpToken => {
                        OpBuilder::default()
                            .mode(self.mode)
                            .limits(true)
                            .parent_is_sup_sub(false)
                            .symbol(true)
                            .name(Some(self.name.to_string()))
                            .build().unwrap().into_node()
                    },
                    NonAtomGroup::TextOrd => {
                        TextOrdBuilder::default()
                            .mode(self.mode)
                            .text(self.name.to_string())
                            .build().unwrap().into_node()
                    },
                    NonAtomGroup::AccentToken => unimplemented!(),
                    NonAtomGroup::Spacing => unimplemented!(),
                }
            }
        }
    }

    pub fn get(mode: Mode, name: char) -> Symbol {
        // Some work arounds since the auto generated code doesn't work perfectly.
        let math_text = "0123456789/@.\"";
        if math_text.contains(name) && mode == Mode::Math {
            return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), name);
        }
        //// --- AUTO GENERATED CODE --- ////
        if mode == Mode::Math && name == '≡' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≡'); }
        if mode == Mode::Math && name == '≺' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≺'); }
        if mode == Mode::Math && name == '≻' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≻'); }
        if mode == Mode::Math && name == '∼' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '∼'); }
        if mode == Mode::Math && name == '⊥' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '⊥'); }
        if mode == Mode::Math && name == '⪯' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⪯'); }
        if mode == Mode::Math && name == '⪰' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⪰'); }
        if mode == Mode::Math && name == '≃' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≃'); }
        if mode == Mode::Math && name == '∣' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '∣'); }
        if mode == Mode::Math && name == '≪' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≪'); }
        if mode == Mode::Math && name == '≫' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≫'); }
        if mode == Mode::Math && name == '≍' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≍'); }
        if mode == Mode::Math && name == '∥' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '∥'); }
        if mode == Mode::Math && name == '⋈' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⋈'); }
        if mode == Mode::Math && name == '⌣' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⌣'); }
        if mode == Mode::Math && name == '⊑' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⊑'); }
        if mode == Mode::Math && name == '⊒' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⊒'); }
        if mode == Mode::Math && name == '≐' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≐'); }
        if mode == Mode::Math && name == '⌢' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⌢'); }
        if mode == Mode::Math && name == '∋' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '∋'); }
        if mode == Mode::Math && name == '∝' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '∝'); }
        if mode == Mode::Math && name == '⊢' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⊢'); }
        if mode == Mode::Math && name == '⊣' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⊣'); }
        if mode == Mode::Math && name == '.' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '.'); }
        if mode == Mode::Math && name == '⋅' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⋅'); }
        if mode == Mode::Math && name == '#' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '#'); }
        if mode == Mode::Text && name == '#' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '#'); }
        if mode == Mode::Math && name == '&' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '&'); }
        if mode == Mode::Text && name == '&' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '&'); }
        if mode == Mode::Math && name == 'ℵ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'ℵ'); }
        if mode == Mode::Math && name == '∀' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '∀'); }
        if mode == Mode::Math && name == 'ℏ' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), 'ℏ'); }
        if mode == Mode::Math && name == '∃' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '∃'); }
        if mode == Mode::Math && name == '∇' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '∇'); }
        if mode == Mode::Math && name == '♭' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '♭'); }
        if mode == Mode::Math && name == 'ℓ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'ℓ'); }
        if mode == Mode::Math && name == '♮' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '♮'); }
        if mode == Mode::Math && name == '♣' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '♣'); }
        if mode == Mode::Math && name == '℘' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '℘'); }
        if mode == Mode::Math && name == '♯' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '♯'); }
        if mode == Mode::Math && name == '♢' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '♢'); }
        if mode == Mode::Math && name == 'ℜ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'ℜ'); }
        if mode == Mode::Math && name == '♡' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '♡'); }
        if mode == Mode::Math && name == 'ℑ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'ℑ'); }
        if mode == Mode::Math && name == '♠' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '♠'); }
        if mode == Mode::Math && name == '§' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '§'); }
        if mode == Mode::Text && name == '§' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '§'); }
        if mode == Mode::Math && name == '¶' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '¶'); }
        if mode == Mode::Text && name == '¶' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '¶'); }
        if mode == Mode::Math && name == '†' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '†'); }
        if mode == Mode::Text && name == '†' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '†'); }
        if mode == Mode::Math && name == '‡' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '‡'); }
        if mode == Mode::Text && name == '‡' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '‡'); }
        if mode == Mode::Math && name == '⎱' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Close), '⎱'); }
        if mode == Mode::Math && name == '⎰' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Open), '⎰'); }
        if mode == Mode::Math && name == '⟯' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Close), '⟯'); }
        if mode == Mode::Math && name == '⟮' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Open), '⟮'); }
        if mode == Mode::Math && name == '∓' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '∓'); }
        if mode == Mode::Math && name == '⊖' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⊖'); }
        if mode == Mode::Math && name == '⊎' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⊎'); }
        if mode == Mode::Math && name == '⊓' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⊓'); }
        if mode == Mode::Math && name == '∗' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '∗'); }
        if mode == Mode::Math && name == '⊔' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⊔'); }
        if mode == Mode::Math && name == '◯' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '◯'); }
        if mode == Mode::Math && name == '∙' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '∙'); }
        if mode == Mode::Math && name == '≀' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '≀'); }
        if mode == Mode::Math && name == '⨿' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⨿'); }
        if mode == Mode::Math && name == '⟵' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⟵'); }
        if mode == Mode::Math && name == '⇐' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⇐'); }
        if mode == Mode::Math && name == '⟸' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⟸'); }
        if mode == Mode::Math && name == '⟶' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⟶'); }
        if mode == Mode::Math && name == '⇒' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⇒'); }
        if mode == Mode::Math && name == '⟹' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⟹'); }
        if mode == Mode::Math && name == '↔' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↔'); }
        if mode == Mode::Math && name == '⟷' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⟷'); }
        if mode == Mode::Math && name == '⇔' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⇔'); }
        if mode == Mode::Math && name == '⟺' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⟺'); }
        if mode == Mode::Math && name == '↦' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↦'); }
        if mode == Mode::Math && name == '⟼' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⟼'); }
        if mode == Mode::Math && name == '↗' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↗'); }
        if mode == Mode::Math && name == '↩' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↩'); }
        if mode == Mode::Math && name == '↪' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↪'); }
        if mode == Mode::Math && name == '↘' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↘'); }
        if mode == Mode::Math && name == '↼' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↼'); }
        if mode == Mode::Math && name == '⇀' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⇀'); }
        if mode == Mode::Math && name == '↙' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↙'); }
        if mode == Mode::Math && name == '↽' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↽'); }
        if mode == Mode::Math && name == '⇁' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⇁'); }
        if mode == Mode::Math && name == '↖' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↖'); }
        if mode == Mode::Math && name == '⇌' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⇌'); }
        if mode == Mode::Math && name == '≮' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≮'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '⪇' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪇'); }
        if mode == Mode::Math && name == '≨' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≨'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '⋦' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋦'); }
        if mode == Mode::Math && name == '⪉' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪉'); }
        if mode == Mode::Math && name == '⊀' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊀'); }
        if mode == Mode::Math && name == '⋠' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋠'); }
        if mode == Mode::Math && name == '⋨' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋨'); }
        if mode == Mode::Math && name == '⪹' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪹'); }
        if mode == Mode::Math && name == '≁' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≁'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '∤' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '∤'); }
        if mode == Mode::Math && name == '⊬' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊬'); }
        if mode == Mode::Math && name == '⊭' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊭'); }
        if mode == Mode::Math && name == '⋪' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋪'); }
        if mode == Mode::Math && name == '⋬' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋬'); }
        if mode == Mode::Math && name == '⊊' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊊'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '⫋' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⫋'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '≯' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≯'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '⪈' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪈'); }
        if mode == Mode::Math && name == '≩' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≩'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '⋧' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋧'); }
        if mode == Mode::Math && name == '⪊' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪊'); }
        if mode == Mode::Math && name == '⊁' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊁'); }
        if mode == Mode::Math && name == '⋡' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋡'); }
        if mode == Mode::Math && name == '⋩' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋩'); }
        if mode == Mode::Math && name == '⪺' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪺'); }
        if mode == Mode::Math && name == '≆' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≆'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '∦' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '∦'); }
        if mode == Mode::Math && name == '⊯' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊯'); }
        if mode == Mode::Math && name == '⋫' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋫'); }
        if mode == Mode::Math && name == '⋭' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋭'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '⊋' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊋'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '⫌' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⫌'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '⊮' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊮'); }
        if mode == Mode::Math && name == '⪵' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪵'); }
        if mode == Mode::Math && name == '⪶' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪶'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '⊴' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊴'); }
        if mode == Mode::Math && name == '⊵' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊵'); }
        if mode == Mode::Math && name == '↚' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↚'); }
        if mode == Mode::Math && name == '↛' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↛'); }
        if mode == Mode::Math && name == '⇍' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇍'); }
        if mode == Mode::Math && name == '⇏' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇏'); }
        if mode == Mode::Math && name == '↮' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↮'); }
        if mode == Mode::Math && name == '⇎' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇎'); }
        if mode == Mode::Math && name == '△' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '△'); }
        if mode == Mode::Math && name == '▽' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '▽'); }
        if mode == Mode::Math && name == '◊' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '◊'); }
        if mode == Mode::Math && name == 'Ⓢ' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), 'Ⓢ'); }
        if mode == Mode::Math && name == '®' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '®'); }
        if mode == Mode::Text && name == '®' { return Symbol::new(Mode::Text, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '®'); }
        if mode == Mode::Math && name == '∡' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '∡'); }
        if mode == Mode::Math && name == '∄' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '∄'); }
        if mode == Mode::Math && name == '℧' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '℧'); }
        if mode == Mode::Math && name == 'Ⅎ' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), 'Ⅎ'); }
        if mode == Mode::Math && name == '⅁' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '⅁'); }
        if mode == Mode::Math && name == '‵' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '‵'); }
        if mode == Mode::Math && name == '▲' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '▲'); }
        if mode == Mode::Math && name == '▼' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '▼'); }
        if mode == Mode::Math && name == '■' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '■'); }
        if mode == Mode::Math && name == '⧫' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '⧫'); }
        if mode == Mode::Math && name == '★' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '★'); }
        if mode == Mode::Math && name == '∢' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '∢'); }
        if mode == Mode::Math && name == '∁' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '∁'); }
        if mode == Mode::Math && name == 'ð' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), 'ð'); }
        if mode == Mode::Text && name == 'ð' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'ð'); }
        if mode == Mode::Math && name == '╱' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '╱'); }
        if mode == Mode::Math && name == '╲' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '╲'); }
        if mode == Mode::Math && name == '□' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '□'); }
        if mode == Mode::Math && name == '¥' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '¥'); }
        if mode == Mode::Text && name == '¥' { return Symbol::new(Mode::Text, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '¥'); }
        if mode == Mode::Math && name == '✓' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '✓'); }
        if mode == Mode::Text && name == '✓' { return Symbol::new(Mode::Text, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '✓'); }
        if mode == Mode::Math && name == 'ℶ' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), 'ℶ'); }
        if mode == Mode::Math && name == 'ℸ' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), 'ℸ'); }
        if mode == Mode::Math && name == 'ℷ' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), 'ℷ'); }
        if mode == Mode::Math && name == 'ϝ' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), 'ϝ'); }
        if mode == Mode::Math && name == 'ϰ' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), 'ϰ'); }
        if mode == Mode::Math && name == '┌' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Open), '┌'); }
        if mode == Mode::Math && name == '┐' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Close), '┐'); }
        if mode == Mode::Math && name == '└' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Open), '└'); }
        if mode == Mode::Math && name == '┘' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Close), '┘'); }
        if mode == Mode::Math && name == '≦' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≦'); }
        if mode == Mode::Math && name == '⩽' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⩽'); }
        if mode == Mode::Math && name == '⪕' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪕'); }
        if mode == Mode::Math && name == '≲' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≲'); }
        if mode == Mode::Math && name == '⪅' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪅'); }
        if mode == Mode::Math && name == '≊' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≊'); }
        if mode == Mode::Math && name == '⋖' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⋖'); }
        if mode == Mode::Math && name == '⋘' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋘'); }
        if mode == Mode::Math && name == '≶' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≶'); }
        if mode == Mode::Math && name == '⋚' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋚'); }
        if mode == Mode::Math && name == '⪋' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪋'); }
        if mode == Mode::Math && name == '≑' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≑'); }
        if mode == Mode::Math && name == '≓' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≓'); }
        if mode == Mode::Math && name == '≒' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≒'); }
        if mode == Mode::Math && name == '∽' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '∽'); }
        if mode == Mode::Math && name == '⋍' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋍'); }
        if mode == Mode::Math && name == '⫅' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⫅'); }
        if mode == Mode::Math && name == '⋐' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋐'); }
        if mode == Mode::Math && name == '⊏' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊏'); }
        if mode == Mode::Math && name == '≼' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≼'); }
        if mode == Mode::Math && name == '⋞' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋞'); }
        if mode == Mode::Math && name == '≾' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≾'); }
        if mode == Mode::Math && name == '⪷' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪷'); }
        if mode == Mode::Math && name == '⊲' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊲'); }
        if mode == Mode::Math && name == '⊨' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⊨'); }
        if mode == Mode::Math && name == '⊪' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊪'); }
        if mode == Mode::Math && name == '≏' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≏'); }
        if mode == Mode::Math && name == '≎' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≎'); }
        if mode == Mode::Math && name == '≧' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≧'); }
        if mode == Mode::Math && name == '⩾' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⩾'); }
        if mode == Mode::Math && name == '⪖' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪖'); }
        if mode == Mode::Math && name == '≳' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≳'); }
        if mode == Mode::Math && name == '⪆' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪆'); }
        if mode == Mode::Math && name == '⋗' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⋗'); }
        if mode == Mode::Math && name == '⋙' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋙'); }
        if mode == Mode::Math && name == '≷' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≷'); }
        if mode == Mode::Math && name == '⋛' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋛'); }
        if mode == Mode::Math && name == '⪌' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪌'); }
        if mode == Mode::Math && name == '≖' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≖'); }
        if mode == Mode::Math && name == '≗' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≗'); }
        if mode == Mode::Math && name == '≜' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≜'); }
        if mode == Mode::Math && name == '≈' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≈'); }
        if mode == Mode::Math && name == '⫆' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⫆'); }
        if mode == Mode::Math && name == '⋑' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋑'); }
        if mode == Mode::Math && name == '⊐' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊐'); }
        if mode == Mode::Math && name == '≽' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≽'); }
        if mode == Mode::Math && name == '⋟' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋟'); }
        if mode == Mode::Math && name == '≿' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≿'); }
        if mode == Mode::Math && name == '⪸' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⪸'); }
        if mode == Mode::Math && name == '⊳' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊳'); }
        if mode == Mode::Math && name == '⊩' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊩'); }
        if mode == Mode::Math && name == '≬' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≬'); }
        if mode == Mode::Math && name == '⋔' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⋔'); }
        if mode == Mode::Math && name == '◀' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '◀'); }
        if mode == Mode::Math && name == '∴' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '∴'); }
        if mode == Mode::Math && name == '∍' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '∍'); }
        if mode == Mode::Math && name == '▶' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '▶'); }
        if mode == Mode::Math && name == '∵' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '∵'); }
        if mode == Mode::Math && name == '≂' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≂'); }
        if mode == Mode::Math && name == '∔' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '∔'); }
        if mode == Mode::Math && name == '∖' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '∖'); }
        if mode == Mode::Math && name == '⋒' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⋒'); }
        if mode == Mode::Math && name == '⋓' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⋓'); }
        if mode == Mode::Math && name == '⩞' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⩞'); }
        if mode == Mode::Math && name == '⊟' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊟'); }
        if mode == Mode::Math && name == '⊞' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊞'); }
        if mode == Mode::Math && name == '⋇' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⋇'); }
        if mode == Mode::Math && name == '⋉' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⋉'); }
        if mode == Mode::Math && name == '⋊' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⋊'); }
        if mode == Mode::Math && name == '⋋' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⋋'); }
        if mode == Mode::Math && name == '⋌' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⋌'); }
        if mode == Mode::Math && name == '⋏' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⋏'); }
        if mode == Mode::Math && name == '⋎' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⋎'); }
        if mode == Mode::Math && name == '⊝' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊝'); }
        if mode == Mode::Math && name == '⊛' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊛'); }
        if mode == Mode::Math && name == '⊺' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊺'); }
        if mode == Mode::Math && name == '⊠' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊠'); }
        if mode == Mode::Math && name == '⇢' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇢'); }
        if mode == Mode::Math && name == '⇠' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇠'); }
        if mode == Mode::Math && name == '⇇' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇇'); }
        if mode == Mode::Math && name == '⇆' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇆'); }
        if mode == Mode::Math && name == '⇚' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇚'); }
        if mode == Mode::Math && name == '↞' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↞'); }
        if mode == Mode::Math && name == '↢' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↢'); }
        if mode == Mode::Math && name == '↫' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↫'); }
        if mode == Mode::Math && name == '⇋' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇋'); }
        if mode == Mode::Math && name == '↶' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↶'); }
        if mode == Mode::Math && name == '↺' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↺'); }
        if mode == Mode::Math && name == '↰' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↰'); }
        if mode == Mode::Math && name == '⇈' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇈'); }
        if mode == Mode::Math && name == '↿' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↿'); }
        if mode == Mode::Math && name == '⇃' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇃'); }
        if mode == Mode::Math && name == '⊶' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⊶'); }
        if mode == Mode::Math && name == '⊷' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⊷'); }
        if mode == Mode::Math && name == '⊸' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊸'); }
        if mode == Mode::Math && name == '↭' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↭'); }
        if mode == Mode::Math && name == '⇉' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇉'); }
        if mode == Mode::Math && name == '⇄' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇄'); }
        if mode == Mode::Math && name == '↠' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↠'); }
        if mode == Mode::Math && name == '↣' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↣'); }
        if mode == Mode::Math && name == '↬' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↬'); }
        if mode == Mode::Math && name == '↷' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↷'); }
        if mode == Mode::Math && name == '↻' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↻'); }
        if mode == Mode::Math && name == '↱' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↱'); }
        if mode == Mode::Math && name == '⇊' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇊'); }
        if mode == Mode::Math && name == '↾' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '↾'); }
        if mode == Mode::Math && name == '⇂' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇂'); }
        if mode == Mode::Math && name == '⇝' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇝'); }
        if mode == Mode::Math && name == '⇛' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⇛'); }
        if mode == Mode::Math && name == '‘' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '‘'); }
        if mode == Mode::Math && name == '$' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '$'); }
        if mode == Mode::Text && name == '$' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '$'); }
        if mode == Mode::Math && name == '%' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '%'); }
        if mode == Mode::Text && name == '%' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '%'); }
        if mode == Mode::Math && name == '_' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '_'); }
        if mode == Mode::Text && name == '_' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '_'); }
        if mode == Mode::Math && name == '∠' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '∠'); }
        if mode == Mode::Math && name == '∞' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '∞'); }
        if mode == Mode::Math && name == '′' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '′'); }
        if mode == Mode::Math && name == 'Γ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Γ'); }
        if mode == Mode::Math && name == 'Δ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Δ'); }
        if mode == Mode::Math && name == 'Θ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Θ'); }
        if mode == Mode::Math && name == 'Λ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Λ'); }
        if mode == Mode::Math && name == 'Ξ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Ξ'); }
        if mode == Mode::Math && name == 'Π' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Π'); }
        if mode == Mode::Math && name == 'Σ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Σ'); }
        if mode == Mode::Math && name == 'Υ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Υ'); }
        if mode == Mode::Math && name == 'Φ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Φ'); }
        if mode == Mode::Math && name == 'Ψ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Ψ'); }
        if mode == Mode::Math && name == 'Ω' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Ω'); }
        if mode == Mode::Math && name == 'A' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'A'); }
        if mode == Mode::Math && name == 'B' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'B'); }
        if mode == Mode::Math && name == 'E' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'E'); }
        if mode == Mode::Math && name == 'Z' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'Z'); }
        if mode == Mode::Math && name == 'H' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'H'); }
        if mode == Mode::Math && name == 'I' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'I'); }
        if mode == Mode::Math && name == 'K' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'K'); }
        if mode == Mode::Math && name == 'M' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'M'); }
        if mode == Mode::Math && name == 'N' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'N'); }
        if mode == Mode::Math && name == 'O' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'O'); }
        if mode == Mode::Math && name == 'P' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'P'); }
        if mode == Mode::Math && name == 'T' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'T'); }
        if mode == Mode::Math && name == 'X' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'X'); }
        if mode == Mode::Math && name == '¬' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '¬'); }
        if mode == Mode::Math && name == '⊤' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '⊤'); }
        if mode == Mode::Math && name == '∅' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '∅'); }
        if mode == Mode::Math && name == 'α' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'α'); }
        if mode == Mode::Math && name == 'β' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'β'); }
        if mode == Mode::Math && name == 'γ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'γ'); }
        if mode == Mode::Math && name == 'δ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'δ'); }
        if mode == Mode::Math && name == 'ϵ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ϵ'); }
        if mode == Mode::Math && name == 'ζ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ζ'); }
        if mode == Mode::Math && name == 'η' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'η'); }
        if mode == Mode::Math && name == 'θ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'θ'); }
        if mode == Mode::Math && name == 'ι' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ι'); }
        if mode == Mode::Math && name == 'κ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'κ'); }
        if mode == Mode::Math && name == 'λ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'λ'); }
        if mode == Mode::Math && name == 'μ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'μ'); }
        if mode == Mode::Math && name == 'ν' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ν'); }
        if mode == Mode::Math && name == 'ξ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ξ'); }
        if mode == Mode::Math && name == 'ο' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ο'); }
        if mode == Mode::Math && name == 'π' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'π'); }
        if mode == Mode::Math && name == 'ρ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ρ'); }
        if mode == Mode::Math && name == 'σ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'σ'); }
        if mode == Mode::Math && name == 'τ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'τ'); }
        if mode == Mode::Math && name == 'υ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'υ'); }
        if mode == Mode::Math && name == 'ϕ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ϕ'); }
        if mode == Mode::Math && name == 'χ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'χ'); }
        if mode == Mode::Math && name == 'ψ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ψ'); }
        if mode == Mode::Math && name == 'ω' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ω'); }
        if mode == Mode::Math && name == 'ε' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ε'); }
        if mode == Mode::Math && name == 'ϑ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ϑ'); }
        if mode == Mode::Math && name == 'ϖ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ϖ'); }
        if mode == Mode::Math && name == 'ϱ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ϱ'); }
        if mode == Mode::Math && name == 'ς' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'ς'); }
        if mode == Mode::Math && name == 'φ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'φ'); }
        if mode == Mode::Math && name == '+' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '+'); }
        if mode == Mode::Math && name == '−' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '−'); }
        if mode == Mode::Math && name == '∘' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '∘'); }
        if mode == Mode::Math && name == '÷' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '÷'); }
        if mode == Mode::Math && name == '±' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '±'); }
        if mode == Mode::Math && name == '×' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '×'); }
        if mode == Mode::Math && name == '∩' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '∩'); }
        if mode == Mode::Math && name == '∪' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '∪'); }
        if mode == Mode::Math && name == '∧' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '∧'); }
        if mode == Mode::Math && name == '∨' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '∨'); }
        if mode == Mode::Math && name == '√' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '√'); }
        if mode == Mode::Math && name == '⟨' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Open), '⟨'); }
        if mode == Mode::Math && name == '?' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Close), '?'); }
        if mode == Mode::Math && name == '!' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Close), '!'); }
        if mode == Mode::Math && name == '⟩' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Close), '⟩'); }
        if mode == Mode::Math && name == '=' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '='); }
        if mode == Mode::Math && name == ':' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), ':'); }
        if mode == Mode::Math && name == '≅' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≅'); }
        if mode == Mode::Math && name == '≥' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≥'); }
        if mode == Mode::Math && name == '←' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '←'); }
        if mode == Mode::Math && name == '>' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '>'); }
        if mode == Mode::Math && name == '∈' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '∈'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), ''); }
        if mode == Mode::Math && name == '⊂' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⊂'); }
        if mode == Mode::Math && name == '⊃' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⊃'); }
        if mode == Mode::Math && name == '⊆' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⊆'); }
        if mode == Mode::Math && name == '⊇' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⊇'); }
        if mode == Mode::Math && name == '⊈' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊈'); }
        if mode == Mode::Math && name == '⊉' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '⊉'); }
        if mode == Mode::Math && name == '≤' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '≤'); }
        if mode == Mode::Math && name == '<' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '<'); }
        if mode == Mode::Math && name == '→' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '→'); }
        if mode == Mode::Math && name == '≱' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≱'); }
        if mode == Mode::Math && name == '≰' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Rel), '≰'); }
        if mode == Mode::Math && name == ' ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::Spacing), ' '); }
        if mode == Mode::Text && name == ' ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::Spacing), ' '); }
        if mode == Mode::Math && name == ',' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Punct), ','); }
        if mode == Mode::Math && name == ';' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Punct), ';'); }
        if mode == Mode::Math && name == '⊼' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊼'); }
        if mode == Mode::Math && name == '⊻' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊻'); }
        if mode == Mode::Math && name == '⊙' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⊙'); }
        if mode == Mode::Math && name == '⊕' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⊕'); }
        if mode == Mode::Math && name == '⊗' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⊗'); }
        if mode == Mode::Math && name == '∂' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '∂'); }
        if mode == Mode::Math && name == '⊘' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⊘'); }
        if mode == Mode::Math && name == '⊚' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊚'); }
        if mode == Mode::Math && name == '⊡' { return Symbol::new(Mode::Math, Font::Ams, Group::Atom(AtomGroup::Bin), '⊡'); }
        if mode == Mode::Math && name == '⋄' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⋄'); }
        if mode == Mode::Math && name == '⋆' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '⋆'); }
        if mode == Mode::Math && name == '◃' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '◃'); }
        if mode == Mode::Math && name == '▹' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Bin), '▹'); }
        if mode == Mode::Math && name == '{' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Open), '{'); }
        if mode == Mode::Text && name == '{' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '{'); }
        if mode == Mode::Math && name == '}' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Close), '}'); }
        if mode == Mode::Text && name == '}' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '}'); }
        if mode == Mode::Math && name == '[' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Open), '['); }
        if mode == Mode::Text && name == '[' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '['); }
        if mode == Mode::Math && name == ']' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Close), ']'); }
        if mode == Mode::Text && name == ']' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), ']'); }
        if mode == Mode::Math && name == '(' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Open), '('); }
        if mode == Mode::Math && name == ')' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Close), ')'); }
        if mode == Mode::Text && name == '<' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '<'); }
        if mode == Mode::Text && name == '>' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '>'); }
        if mode == Mode::Math && name == '⌊' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Open), '⌊'); }
        if mode == Mode::Math && name == '⌋' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Close), '⌋'); }
        if mode == Mode::Math && name == '⌈' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Open), '⌈'); }
        if mode == Mode::Math && name == '⌉' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Close), '⌉'); }
        if mode == Mode::Math && name == '\\' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '\\'); }
        if mode == Mode::Text && name == '|' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '|'); }
        if mode == Mode::Text && name == '∥' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '∥'); }
        if mode == Mode::Text && name == '~' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '~'); }
        if mode == Mode::Text && name == '\\' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '\\'); }
        if mode == Mode::Text && name == '^' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '^'); }
        if mode == Mode::Math && name == '↑' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↑'); }
        if mode == Mode::Math && name == '⇑' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⇑'); }
        if mode == Mode::Math && name == '↓' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↓'); }
        if mode == Mode::Math && name == '⇓' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⇓'); }
        if mode == Mode::Math && name == '↕' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '↕'); }
        if mode == Mode::Math && name == '⇕' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Rel), '⇕'); }
        if mode == Mode::Math && name == '∐' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '∐'); }
        if mode == Mode::Math && name == '⋁' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '⋁'); }
        if mode == Mode::Math && name == '⋀' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '⋀'); }
        if mode == Mode::Math && name == '⨄' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '⨄'); }
        if mode == Mode::Math && name == '⋂' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '⋂'); }
        if mode == Mode::Math && name == '⋃' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '⋃'); }
        if mode == Mode::Math && name == '∫' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '∫'); }
        if mode == Mode::Math && name == '∬' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '∬'); }
        if mode == Mode::Math && name == '∭' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '∭'); }
        if mode == Mode::Math && name == '∏' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '∏'); }
        if mode == Mode::Math && name == '∑' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '∑'); }
        if mode == Mode::Math && name == '⨂' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '⨂'); }
        if mode == Mode::Math && name == '⨁' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '⨁'); }
        if mode == Mode::Math && name == '⨀' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '⨀'); }
        if mode == Mode::Math && name == '∮' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '∮'); }
        if mode == Mode::Math && name == '∯' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '∯'); }
        if mode == Mode::Math && name == '∰' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '∰'); }
        if mode == Mode::Math && name == '⨆' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::OpToken), '⨆'); }
        if mode == Mode::Text && name == '…' { return Symbol::new(Mode::Text, Font::Main, Group::Atom(AtomGroup::Inner), '…'); }
        if mode == Mode::Math && name == '…' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Inner), '…'); }
        if mode == Mode::Math && name == '⋯' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Inner), '⋯'); }
        if mode == Mode::Math && name == '⋱' { return Symbol::new(Mode::Math, Font::Main, Group::Atom(AtomGroup::Inner), '⋱'); }
        if mode == Mode::Math && name == '⋮' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '⋮'); }
        if mode == Mode::Math && name == 'ˊ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), 'ˊ'); }
        if mode == Mode::Math && name == 'ˋ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), 'ˋ'); }
        if mode == Mode::Math && name == '¨' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '¨'); }
        if mode == Mode::Math && name == '~' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '~'); }
        if mode == Mode::Math && name == 'ˉ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), 'ˉ'); }
        if mode == Mode::Math && name == '˘' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '˘'); }
        if mode == Mode::Math && name == 'ˇ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), 'ˇ'); }
        if mode == Mode::Math && name == '^' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '^'); }
        if mode == Mode::Math && name == '⃗' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '⃗'); }
        if mode == Mode::Math && name == '˙' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '˙'); }
        if mode == Mode::Math && name == '˚' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '˚'); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), ''); }
        if mode == Mode::Math && name == '' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), ''); }
        if mode == Mode::Math && name == 'ı' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'ı'); }
        if mode == Mode::Math && name == 'ȷ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'ȷ'); }
        if mode == Mode::Text && name == 'ı' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'ı'); }
        if mode == Mode::Text && name == 'ȷ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'ȷ'); }
        if mode == Mode::Text && name == 'ß' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'ß'); }
        if mode == Mode::Text && name == 'æ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'æ'); }
        if mode == Mode::Text && name == 'œ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'œ'); }
        if mode == Mode::Text && name == 'ø' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'ø'); }
        if mode == Mode::Text && name == 'Æ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Æ'); }
        if mode == Mode::Text && name == 'Œ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Œ'); }
        if mode == Mode::Text && name == 'Ø' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Ø'); }
        if mode == Mode::Text && name == 'ˊ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), 'ˊ'); }
        if mode == Mode::Text && name == 'ˋ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), 'ˋ'); }
        if mode == Mode::Text && name == 'ˆ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), 'ˆ'); }
        if mode == Mode::Text && name == '˜' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '˜'); }
        if mode == Mode::Text && name == 'ˉ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), 'ˉ'); }
        if mode == Mode::Text && name == '˘' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '˘'); }
        if mode == Mode::Text && name == '˙' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '˙'); }
        if mode == Mode::Text && name == '¸' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '¸'); }
        if mode == Mode::Text && name == '˚' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '˚'); }
        if mode == Mode::Text && name == 'ˇ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), 'ˇ'); }
        if mode == Mode::Text && name == '¨' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '¨'); }
        if mode == Mode::Text && name == '˝' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '˝'); }
        if mode == Mode::Text && name == '◯' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::AccentToken), '◯'); }
        if mode == Mode::Text && name == '–' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '–'); }
        if mode == Mode::Text && name == '—' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '—'); }
        if mode == Mode::Text && name == '‘' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '‘'); }
        if mode == Mode::Text && name == '’' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '’'); }
        if mode == Mode::Text && name == '“' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '“'); }
        if mode == Mode::Text && name == '”' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '”'); }
        if mode == Mode::Math && name == '°' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '°'); }
        if mode == Mode::Text && name == '°' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '°'); }
        if mode == Mode::Math && name == '£' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '£'); }
        if mode == Mode::Text && name == '£' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '£'); }
        if mode == Mode::Math && name == '✠' { return Symbol::new(Mode::Math, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '✠'); }
        if mode == Mode::Text && name == '✠' { return Symbol::new(Mode::Text, Font::Ams, Group::NonAtom(NonAtomGroup::TextOrd), '✠'); }
        if mode == Mode::Math && name == '0' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), '0'); }
        if mode == Mode::Math && name == '1' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), '1'); }
        if mode == Mode::Math && name == '2' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), '2'); }
        if mode == Mode::Math && name == '3' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), '3'); }
        if mode == Mode::Math && name == '4' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), '4'); }
        if mode == Mode::Math && name == '5' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), '5'); }
        if mode == Mode::Math && name == '6' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), '6'); }
        if mode == Mode::Math && name == '7' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), '7'); }
        if mode == Mode::Math && name == '8' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), '8'); }
        if mode == Mode::Math && name == '9' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), '9'); }
        if mode == Mode::Math && name == '/' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '/'); }
        if mode == Mode::Math && name == '@' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '@'); }
        if mode == Mode::Math && name == '"' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '"'); }
        if mode == Mode::Text && name == '0' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '0'); }
        if mode == Mode::Text && name == '1' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '1'); }
        if mode == Mode::Text && name == '2' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '2'); }
        if mode == Mode::Text && name == '3' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '3'); }
        if mode == Mode::Text && name == '4' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '4'); }
        if mode == Mode::Text && name == '5' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '5'); }
        if mode == Mode::Text && name == '6' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '6'); }
        if mode == Mode::Text && name == '7' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '7'); }
        if mode == Mode::Text && name == '8' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '8'); }
        if mode == Mode::Text && name == '9' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '9'); }
        if mode == Mode::Text && name == '!' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '!'); }
        if mode == Mode::Text && name == '@' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '@'); }
        if mode == Mode::Text && name == '*' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '*'); }
        if mode == Mode::Text && name == '(' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '('); }
        if mode == Mode::Text && name == ')' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), ')'); }
        if mode == Mode::Text && name == '-' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '-'); }
        if mode == Mode::Text && name == '=' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '='); }
        if mode == Mode::Text && name == '+' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '+'); }
        if mode == Mode::Text && name == '"' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '"'); }
        if mode == Mode::Text && name == ';' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), ';'); }
        if mode == Mode::Text && name == ':' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), ':'); }
        if mode == Mode::Text && name == '?' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '?'); }
        if mode == Mode::Text && name == '/' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '/'); }
        if mode == Mode::Text && name == '.' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), '.'); }
        if mode == Mode::Text && name == ',' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), ','); }
        if mode == Mode::Text && name == 'A' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'A'); }
        if mode == Mode::Text && name == 'B' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'B'); }
        if mode == Mode::Math && name == 'C' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'C'); }
        if mode == Mode::Text && name == 'C' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'C'); }
        if mode == Mode::Math && name == 'D' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'D'); }
        if mode == Mode::Text && name == 'D' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'D'); }
        if mode == Mode::Text && name == 'E' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'E'); }
        if mode == Mode::Math && name == 'F' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'F'); }
        if mode == Mode::Text && name == 'F' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'F'); }
        if mode == Mode::Math && name == 'G' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'G'); }
        if mode == Mode::Text && name == 'G' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'G'); }
        if mode == Mode::Text && name == 'H' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'H'); }
        if mode == Mode::Text && name == 'I' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'I'); }
        if mode == Mode::Math && name == 'J' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'J'); }
        if mode == Mode::Text && name == 'J' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'J'); }
        if mode == Mode::Text && name == 'K' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'K'); }
        if mode == Mode::Math && name == 'L' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'L'); }
        if mode == Mode::Text && name == 'L' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'L'); }
        if mode == Mode::Text && name == 'M' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'M'); }
        if mode == Mode::Text && name == 'N' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'N'); }
        if mode == Mode::Text && name == 'O' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'O'); }
        if mode == Mode::Text && name == 'P' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'P'); }
        if mode == Mode::Math && name == 'Q' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'Q'); }
        if mode == Mode::Text && name == 'Q' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Q'); }
        if mode == Mode::Math && name == 'R' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'R'); }
        if mode == Mode::Text && name == 'R' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'R'); }
        if mode == Mode::Math && name == 'S' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'S'); }
        if mode == Mode::Text && name == 'S' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'S'); }
        if mode == Mode::Text && name == 'T' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'T'); }
        if mode == Mode::Math && name == 'U' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'U'); }
        if mode == Mode::Text && name == 'U' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'U'); }
        if mode == Mode::Math && name == 'V' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'V'); }
        if mode == Mode::Text && name == 'V' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'V'); }
        if mode == Mode::Math && name == 'W' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'W'); }
        if mode == Mode::Text && name == 'W' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'W'); }
        if mode == Mode::Text && name == 'X' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'X'); }
        if mode == Mode::Math && name == 'Y' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'Y'); }
        if mode == Mode::Text && name == 'Y' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Y'); }
        if mode == Mode::Text && name == 'Z' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Z'); }
        if mode == Mode::Math && name == 'a' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'a'); }
        if mode == Mode::Text && name == 'a' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'a'); }
        if mode == Mode::Math && name == 'b' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'b'); }
        if mode == Mode::Text && name == 'b' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'b'); }
        if mode == Mode::Math && name == 'c' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'c'); }
        if mode == Mode::Text && name == 'c' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'c'); }
        if mode == Mode::Math && name == 'd' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'd'); }
        if mode == Mode::Text && name == 'd' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'd'); }
        if mode == Mode::Math && name == 'e' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'e'); }
        if mode == Mode::Text && name == 'e' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'e'); }
        if mode == Mode::Math && name == 'f' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'f'); }
        if mode == Mode::Text && name == 'f' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'f'); }
        if mode == Mode::Math && name == 'g' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'g'); }
        if mode == Mode::Text && name == 'g' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'g'); }
        if mode == Mode::Math && name == 'h' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'h'); }
        if mode == Mode::Text && name == 'h' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'h'); }
        if mode == Mode::Math && name == 'i' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'i'); }
        if mode == Mode::Text && name == 'i' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'i'); }
        if mode == Mode::Math && name == 'j' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'j'); }
        if mode == Mode::Text && name == 'j' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'j'); }
        if mode == Mode::Math && name == 'k' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'k'); }
        if mode == Mode::Text && name == 'k' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'k'); }
        if mode == Mode::Math && name == 'l' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'l'); }
        if mode == Mode::Text && name == 'l' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'l'); }
        if mode == Mode::Math && name == 'm' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'm'); }
        if mode == Mode::Text && name == 'm' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'm'); }
        if mode == Mode::Math && name == 'n' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'n'); }
        if mode == Mode::Text && name == 'n' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'n'); }
        if mode == Mode::Math && name == 'o' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'o'); }
        if mode == Mode::Text && name == 'o' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'o'); }
        if mode == Mode::Math && name == 'p' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'p'); }
        if mode == Mode::Text && name == 'p' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'p'); }
        if mode == Mode::Math && name == 'q' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'q'); }
        if mode == Mode::Text && name == 'q' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'q'); }
        if mode == Mode::Math && name == 'r' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'r'); }
        if mode == Mode::Text && name == 'r' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'r'); }
        if mode == Mode::Math && name == 's' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 's'); }
        if mode == Mode::Text && name == 's' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 's'); }
        if mode == Mode::Math && name == 't' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 't'); }
        if mode == Mode::Text && name == 't' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 't'); }
        if mode == Mode::Math && name == 'u' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'u'); }
        if mode == Mode::Text && name == 'u' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'u'); }
        if mode == Mode::Math && name == 'v' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'v'); }
        if mode == Mode::Text && name == 'v' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'v'); }
        if mode == Mode::Math && name == 'w' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'w'); }
        if mode == Mode::Text && name == 'w' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'w'); }
        if mode == Mode::Math && name == 'x' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'x'); }
        if mode == Mode::Text && name == 'x' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'x'); }
        if mode == Mode::Math && name == 'y' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'y'); }
        if mode == Mode::Text && name == 'y' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'y'); }
        if mode == Mode::Math && name == 'z' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'z'); }
        if mode == Mode::Text && name == 'z' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'z'); }
        if mode == Mode::Math && name == 'Ð' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'Ð'); }
        if mode == Mode::Text && name == 'Ð' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Ð'); }
        if mode == Mode::Math && name == 'Þ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'Þ'); }
        if mode == Mode::Text && name == 'Þ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'Þ'); }
        if mode == Mode::Math && name == 'þ' { return Symbol::new(Mode::Math, Font::Main, Group::NonAtom(NonAtomGroup::MathOrd), 'þ'); }
        if mode == Mode::Text && name == 'þ' { return Symbol::new(Mode::Text, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), 'þ'); }
        //// --------------------------- ////
        return Symbol::new(mode, Font::Main, Group::NonAtom(NonAtomGroup::TextOrd), name);
    }
}
