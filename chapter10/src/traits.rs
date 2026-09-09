// (#)traits.rs 0.1.0   09/08/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates traits in Rust

/// Summary trait
pub trait Summary {
    /// Summarize the item
    /// 
    /// # Returns
    /// 
    /// * `String` - A string representation of the item
    fn summarize(&self) -> String;

    /// Categorize the item
    /// 
    /// # Returns
    /// 
    /// * `String` - A string representation of the item category
    fn categorize(&self) -> String {
        String::from("Setting the default category")
    }
}

/// News Article struct
/// 
/// # Fields
/// 
/// * `headline` - The headline of the news article
/// * `location` - The location of the news article
/// * `author` - The author of the news article
/// * `content` - The content of the news article
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/// Implementation of Summary trait for NewsArticle
impl Summary for NewsArticle {
    /// Summarize the news article
    /// 
    /// # Returns
    /// 
    /// * `String` - A string representation of the news article
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

/// Social Post struct
/// 
/// # Fields
/// 
/// * `username` - The username of the social media user
/// * `content` - The content of the social media post
/// * `reply` - Whether the post is a reply
/// * `repost` - Whether the post is a repost
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

/// Implementation of Summary trait for SocialPost
impl Summary for SocialPost {
    /// Summarize the social media post
    /// 
    /// # Returns
    /// 
    /// * `String` - A string representation of the social media post
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/// The traits function
pub fn traits() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("New post available: {}", post.summarize());
    println!("Reply: {}; Repost: {}", post.reply, post.repost);
    println!("Category: {}", post.categorize());

    notify(&post);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    println!("Content: {}", article.content);
    println!("Category: {}", article.categorize());
    
    notify(&article);
}

/// Notify function
/// 
/// # Arguments
/// 
/// * `item` - A reference to an item that implements the Summary trait
/// 
/// # Returns
/// 
/// * `()` - No return value
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
