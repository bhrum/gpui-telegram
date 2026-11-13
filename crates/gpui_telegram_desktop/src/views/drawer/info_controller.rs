//! Info Controller
//! Upstream: Telegram/SourceFiles/info/info_controller.h, Telegram/SourceFiles/info/info_controller.cpp

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfoSectionType {
    Profile,
    Media,
    CommonGroups,
    Downloads,
    Members,
    Settings,
    Stories,
    Reactions,
    PollResults,
    CommunityRequests,
    SimilarPeers,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfoKey {
    pub peer_id: u64,
    pub topic_id: u64,
}

impl InfoKey {
    pub fn new(peer_id: u64, topic_id: u64) -> Self {
        Self { peer_id, topic_id }
    }

    pub fn peer(peer_id: u64) -> Self {
        Self { peer_id, topic_id: 0 }
    }

    pub fn is_topic(&self) -> bool {
        self.topic_id != 0
    }
}

#[derive(Debug, Clone)]
pub struct InfoController {
    pub key: InfoKey,
    pub current_section: InfoSectionType,
    pub stack: Vec<InfoSectionType>,
    pub max_stack_depth: usize,
}

impl Default for InfoController {
    fn default() -> Self {
        Self {
            key: InfoKey::new(0, 0),
            current_section: InfoSectionType::Profile,
            stack: Vec::new(),
            max_stack_depth: 32,
        }
    }
}

impl InfoController {
    pub fn new(key: InfoKey) -> Self {
        Self {
            key,
            current_section: InfoSectionType::Profile,
            stack: Vec::new(),
            max_stack_depth: 32,
        }
    }

    pub fn show_section(&mut self, section: InfoSectionType) {
        if self.current_section != section {
            if self.stack.len() < self.max_stack_depth {
                self.stack.push(self.current_section);
            }
            self.current_section = section;
        }
    }

    pub fn can_go_back(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn go_back(&mut self) -> Option<InfoSectionType> {
        if let Some(prev) = self.stack.pop() {
            self.current_section = prev;
            Some(prev)
        } else {
            None
        }
    }

    pub fn clear_stack(&mut self) {
        self.stack.clear();
    }

    pub fn set_key(&mut self, key: InfoKey) {
        self.key = key;
        self.clear_stack();
        self.current_section = InfoSectionType::Profile;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_controller_navigation() {
        let mut controller = InfoController::new(InfoKey::peer(12345));
        assert_eq!(controller.current_section, InfoSectionType::Profile);
        assert!(!controller.can_go_back());

        controller.show_section(InfoSectionType::Media);
        assert_eq!(controller.current_section, InfoSectionType::Media);
        assert!(controller.can_go_back());

        controller.show_section(InfoSectionType::Downloads);
        assert_eq!(controller.current_section, InfoSectionType::Downloads);

        assert_eq!(controller.go_back(), Some(InfoSectionType::Media));
        assert_eq!(controller.go_back(), Some(InfoSectionType::Profile));
        assert!(!controller.can_go_back());
    }

    #[test]
    fn test_info_key() {
        let key = InfoKey::new(100, 5);
        assert_eq!(key.peer_id, 100);
        assert_eq!(key.topic_id, 5);
        assert!(key.is_topic());

        let peer_only = InfoKey::peer(200);
        assert!(!peer_only.is_topic());
    }
}
