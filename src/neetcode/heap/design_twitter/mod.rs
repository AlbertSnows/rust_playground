use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Twitter {
    tweets: HashMap<i32, Vec<(i32, i32)>>,
    following: HashMap<i32, HashSet<i32>>,
    timestamp: i32,
}

impl Twitter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        Default::default()
    }

    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        Default::default()
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        Default::default()
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
        twitter.follow(1, 2);
        twitter.post_tweet(2, 6);
        assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
        twitter.unfollow(1, 2);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
    }

    #[test]
    fn empty_feed() {
        let twitter = Twitter::new();
        assert_eq!(twitter.get_news_feed(1), vec![] as Vec<i32>);
    }
}
