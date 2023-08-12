// pub(crate) fn is_valid(topic: &str) -> bool {
//     if topic.is_empty() {
//         false
//     } else {
//         enum PrevState {
//             None,
//             LevelSep,
//             SingleWildcard,
//             MultiWildcard,
//             Other,
//         }

//         let mut previous = PrevState::None;
//         for current in topic.bytes() {
//             previous = match (current, &previous) {
//                 (_, PrevState::MultiWildcard) => return false, // `#` is not last char
//                 (b'+', PrevState::None | PrevState::LevelSep) => PrevState::SingleWildcard,
//                 (b'#', PrevState::None | PrevState::LevelSep) => PrevState::MultiWildcard,
//                 (b'+' | b'#', _) => return false, // `+` or `#` after char other than `/`
//                 (b'/', _) => PrevState::LevelSep,
//                 (_, PrevState::SingleWildcard) => return false, // `+` is followed by char other than `/`
//                 _ => PrevState::Other,
//             }
//         }
//         true
//     }
// }

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TopicFilter(Vec<TopicFilterLevel>);

// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// pub enum TopicFilterError {
//     InvalidTopic,
//     InvalidLevel,
// }

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TopicFilterLevel {
    Normal,
    System,
    Blank,
    SingleWildcard, // Single level wildcard +
    MultiWildcard,  // Multi-level wildcard #
}
